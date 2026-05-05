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
    shared::{DomainElement, error::DomainError},
};

/// Represents a token that can refresh a [User]'s session.
#[derive(Clone)]
pub struct RefreshToken {
    /// The ID of this [`RefreshToken`].
    id: Option<i32>,

    /// The creation date of this [`RefreshToken`].
    created_at: Option<OffsetDateTime>,

    /// The date of the last update of this [`RefreshToken`].
    last_updated_at: Option<OffsetDateTime>,

    /// The value of this [`RefreshToken`].
    value: RefreshTokenValue,

    /// The User ID of this [`RefreshToken`].
    user_id: i32,

    /// The expiration date of this [`RefreshToken`].
    expires_at: OffsetDateTime,
}

impl RefreshToken {
    /// Creates a new [`RefreshToken`].
    pub fn new(user_id: i32, created_at: OffsetDateTime, expires_at: OffsetDateTime) -> Self {
        RefreshToken {
            id: None,
            created_at: Some(created_at),
            last_updated_at: None,
            value: RefreshTokenValue::default(),
            user_id,
            expires_at,
        }
    }

    /// Recreates a [`RefreshToken`].
    pub fn from(
        id: i32,
        created_at: OffsetDateTime,
        last_updated_at: Option<OffsetDateTime>,
        value: impl Into<String>,
        user_id: i32,
        expires_at: OffsetDateTime,
    ) -> Self {
        RefreshToken {
            id: Some(id),
            created_at: Some(created_at),
            last_updated_at,
            value: RefreshTokenValue::from(value),
            user_id,
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
        self.expires_at - self.created_at.unwrap()
    }
}

impl DomainElement<AuthenticationErrorKind> for RefreshToken {
    /// Returns the ID of this [`RefreshToken`].
    fn id(&self) -> Result<i32, DomainError<AuthenticationErrorKind>> {
        self.id
            .ok_or(DomainError::from(AuthenticationErrorKind::Refresh(
                RefreshErrorKind::NotPersistedYet,
            )))
    }

    /// Gets the [creation date][`OffsetDateTime`] of this [`RefreshToken`].
    fn created_at(&self) -> &Option<OffsetDateTime> {
        &self.created_at
    }

    /// Gets the [latest update date][`OffsetDateTime`] of this [`RefreshToken`].
    fn last_updated_at(&self) -> &Option<OffsetDateTime> {
        &self.last_updated_at
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
