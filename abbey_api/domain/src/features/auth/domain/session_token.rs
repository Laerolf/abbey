use axum::extract::FromRequestParts;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tracing::{debug, error};

use crate::{
    features::auth::{
        domain::AuthenticationTokens,
        error::{AuthenticationErrorKind, LoginErrorKind},
    },
    shared::error::DomainError,
};

/// The prefix of the [`SessionToken`][AuthenticationTokens::SessionToken] header value.
pub const SESSION_TOKEN_HEADER_PREFIX: &str = "Bearer ";

/// The claims of a [`SessionToken`].
/// See: https://github.com/Keats/jsonwebtoken
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionTokenClaims {
    /// The subject (whom token refers to).
    sub: i32,
    /// The game the session is associated with.
    game_id: Option<i32>,
    /// The expiration time (as UTC timestamp).
    exp: usize,
    /// Issued at (as UTC timestamp).
    iat: usize,
}

impl SessionTokenClaims {
    pub fn new(
        subject: i32,
        game_id: Option<i32>,
        expires_at: OffsetDateTime,
        issued_at: OffsetDateTime,
    ) -> Result<Self, DomainError<AuthenticationErrorKind>> {
        let exp = expires_at
            .date()
            .with_hms_milli(
                expires_at.hour(),
                expires_at.minute(),
                expires_at.second(),
                0,
            )
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Login(
                    LoginErrorKind::CreateSessionToken,
                ))
                .with_cause(error)
            })?
            .as_utc()
            .unix_timestamp() as usize;

        let iat = issued_at
            .date()
            .with_hms_milli(issued_at.hour(), issued_at.minute(), issued_at.second(), 0)
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Login(
                    LoginErrorKind::CreateSessionToken,
                ))
                .with_cause(error)
            })?
            .as_utc()
            .unix_timestamp() as usize;

        Ok(Self {
            sub: subject,
            game_id,
            exp,
            iat,
        })
    }
}

/// Represents a token of a [User]'s session.
#[derive(Clone, Debug)]
pub struct SessionToken {
    /// The user ID of the session token.
    user_id: i32,
    /// The game ID of the session token.
    game_id: Option<i32>,
    /// The time the session token was created.
    created_at: OffsetDateTime,
    /// The time the session token expires.
    expires_at: OffsetDateTime,
}

impl SessionToken {
    /// Creates a new [`SessionToken`].
    pub fn new(
        user_id: i32,
        game_id: Option<i32>,
        created_at: OffsetDateTime,
        expires_at: OffsetDateTime,
    ) -> Self {
        Self {
            user_id,
            game_id,
            created_at,
            expires_at,
        }
    }

    /// Returns the User ID of this [`SessionToken`].
    pub fn user_id(&self) -> &i32 {
        &self.user_id
    }

    /// Returns the Game ID of this [`SessionToken`].
    pub fn game_id(&self) -> &Option<i32> {
        &self.game_id
    }

    /// Creates a [`SessionToken`] from a JWT.
    pub fn from(
        jwt: impl Into<String>,
    ) -> Result<SessionToken, DomainError<AuthenticationErrorKind>> {
        let jwt_secret = std::env::var("JWT_SECRET")
            .inspect_err(|error| error!("Failed to find the JWT secret => {}", error))
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Authenticate).with_cause(error)
            })?;

        let token_data = decode::<SessionTokenClaims>(
            &jwt.into(),
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .inspect_err(|error| error!("Failed to decode the provided JWT => {}", error))
        .map_err(|error| {
            DomainError::from(AuthenticationErrorKind::Authenticate).with_cause(error)
        })?;

        let expires_at: OffsetDateTime = OffsetDateTime::from_unix_timestamp(
            token_data
                .claims
                .exp
                .try_into()
                .inspect_err(|error| {
                    error!(
                        "Failed to parse the expiration time of a session token: '{}'",
                        error
                    )
                })
                .map_err(|error| {
                    DomainError::from(AuthenticationErrorKind::Authenticate).with_cause(error)
                })?,
        )
        .inspect_err(|error| {
            error!(
                "Failed to parse the expiration time of a session token: '{}'",
                error
            )
        })
        .map_err(|error| {
            DomainError::from(AuthenticationErrorKind::Authenticate).with_cause(error)
        })?;

        let created_at: OffsetDateTime = OffsetDateTime::from_unix_timestamp(
            token_data
                .claims
                .iat
                .try_into()
                .inspect_err(|error| {
                    error!(
                        "Failed to parse the creation time of a session token: '{}'",
                        error
                    )
                })
                .map_err(|error| {
                    DomainError::from(AuthenticationErrorKind::Authenticate).with_cause(error)
                })?,
        )
        .inspect_err(|error| {
            error!(
                "Failed to parse the creation time of a session token: '{}'",
                error
            )
        })
        .map_err(|error| {
            DomainError::from(AuthenticationErrorKind::Authenticate).with_cause(error)
        })?;

        debug!("Created a session token.");

        Ok(Self {
            user_id: token_data.claims.sub,
            game_id: token_data.claims.game_id,
            expires_at,
            created_at,
        })
    }

    /// Converts a [`SessionToken`] to a JWT.
    pub fn to_jwt(&self) -> Result<String, DomainError<AuthenticationErrorKind>> {
        let jwt_secret = std::env::var("JWT_SECRET").map_err(|error| {
            DomainError::from(AuthenticationErrorKind::Login(
                LoginErrorKind::CreateSessionToken,
            ))
            .with_cause(error)
        })?;

        jsonwebtoken::encode(
            &Header::default(),
            &SessionTokenClaims::new(self.user_id, self.game_id, self.expires_at, self.created_at)
                .map_err(|error| {
                    DomainError::from(AuthenticationErrorKind::Login(
                        LoginErrorKind::CreateSessionToken,
                    ))
                    .with_cause(error)
                })?,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )
        .map_err(|error| {
            DomainError::from(AuthenticationErrorKind::Login(
                LoginErrorKind::CreateSessionToken,
            ))
            .with_cause(error)
        })
    }

    /// Validates if this [`SessionToken`] is valid.
    pub fn validate(self) -> Result<Self, DomainError<AuthenticationErrorKind>> {
        if OffsetDateTime::now_utc() > self.expires_at {
            return Err(DomainError::from(AuthenticationErrorKind::Authenticate));
        }

        Ok(self)
    }
}

impl<S> FromRequestParts<S> for SessionToken
where
    S: Send + Sync,
{
    type Rejection = DomainError<AuthenticationErrorKind>;

    async fn from_request_parts(
        request: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let parsed_jwt = request
            .headers
            .get(AuthenticationTokens::SessionToken.header_name())
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix(SESSION_TOKEN_HEADER_PREFIX))
            .map(|value| value.to_string())
            .ok_or(DomainError::from(AuthenticationErrorKind::Authenticate))
            .inspect_err(|error| error!("No JWT was provided => {}", error))?;

        SessionToken::from(parsed_jwt)
            .inspect_err(|error| {
                error!(
                    "Failed to parse a session token from a request's JWT: '{}'",
                    error
                )
            })
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Authenticate).with_cause(error)
            })?
            .validate()
    }
}
