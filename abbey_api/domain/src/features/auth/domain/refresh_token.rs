use axum::extract::FromRequestParts;
use base64::{Engine, engine::general_purpose};
use rand::{Rng, thread_rng};
use time::{Duration, OffsetDateTime};
use tower_cookies::Cookies;
use tracing::{debug, error};

use crate::{
    features::auth::{
        domain::AuthenticationTokens,
        error::{AuthenticationErrorKind, RefreshErrorKind},
    },
    shared::error::DomainError,
};

/// Represents a token that can refresh a [User]'s session.
#[derive(Clone)]
pub struct RefreshToken {
    /// The ID of the refresh token.
    id: Option<i32>,
    /// The value of the refresh token.
    value: RefreshTokenValue,
    /// The User ID of the refresh token.
    user_id: i32,
    /// The time the session refresh was created.
    created_at: OffsetDateTime,
    /// The time the session refresh expires.
    expires_at: OffsetDateTime,
}

impl RefreshToken {
    /// Creates a new [`RefreshToken`].
    pub fn new(user_id: i32, created_at: OffsetDateTime, expires_at: OffsetDateTime) -> Self {
        RefreshToken {
            id: None,
            value: RefreshTokenValue::default(),
            user_id,
            created_at,
            expires_at,
        }
    }

    /// Recreates a [`RefreshToken`].
    pub fn from(
        id: i32,
        value: impl Into<String>,
        user_id: i32,
        created_at: OffsetDateTime,
        expires_at: OffsetDateTime,
    ) -> Self {
        RefreshToken {
            id: Some(id),
            value: RefreshTokenValue::from(value),
            user_id,
            created_at,
            expires_at,
        }
    }

    /// Validates if this [`RefreshToken`] is valid.
    pub fn validate(self) -> Result<Self, DomainError<AuthenticationErrorKind>> {
        if OffsetDateTime::now_utc() > self.expires_at {
            return Err(DomainError::from(AuthenticationErrorKind::Authenticate));
        }

        Ok(self)
    }

    /// Returns the ID of this [`RefreshToken`].
    pub fn id(&self) -> &Option<i32> {
        &self.id
    }

    /// Returns the User ID of this [`RefreshToken`].
    pub fn user_id(&self) -> &i32 {
        &self.user_id
    }

    /// Returns the [value][`RefreshTokenValue`] of this [`RefreshToken`].
    pub fn value(&self) -> &RefreshTokenValue {
        &self.value
    }

    /// Computes the lifespan of this [`RefreshToken`].
    pub fn lifespan(&self) -> Duration {
        self.expires_at - self.created_at
    }
}

#[derive(Clone, Debug)]
pub struct RefreshTokenValue {
    value: String,
}

impl RefreshTokenValue {
    /// Creates a [`RefreshTokenValue`] based on a provided value.
    pub fn from(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    fn generate_value() -> String {
        let mut range = thread_rng();
        let mut bytes = [0u8; 32];

        range.fill(&mut bytes);

        general_purpose::URL_SAFE_NO_PAD.encode(bytes)
    }

    pub fn to_string(&self) -> &String {
        &self.value
    }
}

impl Default for RefreshTokenValue {
    /// Creates a new [`RefreshTokenValue`].
    fn default() -> Self {
        Self {
            value: Self::generate_value(),
        }
    }
}

impl<S> FromRequestParts<S> for RefreshTokenValue
where
    S: Send + Sync,
{
    type Rejection = DomainError<AuthenticationErrorKind>;

    async fn from_request_parts(
        request: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(request, state)
            .await
            .inspect_err(|_error| error!("Failed to find the refresh token cookie."))
            .map_err(|_error| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::RefreshTokenNotFound,
                ))
            })?;

        let refresh_token_value: String = cookies
            .get(&AuthenticationTokens::RefreshToken.cookie_name())
            .and_then(|cookie| cookie.value().parse().ok())
            .ok_or(DomainError::from(AuthenticationErrorKind::Refresh(
                RefreshErrorKind::RefreshTokenNotFound,
            )))
            .inspect_err(|error| error!("Failed to get the refresh token value => {}", error))?;

        debug!("Created a refresh token.");

        Ok(RefreshTokenValue::from(refresh_token_value))
    }
}
