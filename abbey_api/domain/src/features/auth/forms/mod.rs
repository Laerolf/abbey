use time::OffsetDateTime;

use crate::features::auth::domain::refresh_token::RefreshTokenValue;

/// Represents a user account registration form.
#[derive(Clone)]
pub struct RegistrationForm {
    /// The email address of the new user account to create.
    pub email: String,
    /// The password of the new user account to create.
    pub password: String,
}

impl RegistrationForm {
    /// Creates a new [`RegistrationForm`].
    pub fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            password: password.into(),
        }
    }
}

/// Represents a [`RefreshToken`][`super::domain::SessionToken`] creation form.
pub struct RefreshTokenCreationForm {
    /// The user ID of the [`RefreshToken`][`super::domain::RefreshToken`] to create.
    pub user_id: i32,
    /// The value of the [`RefreshToken`][`super::domain::RefreshToken`] to create.
    pub value: String,
    /// The creation time of the [`RefreshToken`][`super::domain::RefreshToken`] to create.
    pub created_at: OffsetDateTime,
    /// The expiration time of the [`RefreshToken`][`super::domain::RefreshToken`] to create.
    pub expires_at: OffsetDateTime,
}

impl RefreshTokenCreationForm {
    /// Creates a new [`RefreshTokenCreationForm`].
    pub fn new(user_id: i32, created_at: OffsetDateTime, expires_at: OffsetDateTime) -> Self {
        Self {
            user_id,
            value: RefreshTokenValue::default().to_string().to_owned(),
            created_at,
            expires_at,
        }
    }
}

/// Represents a user login attempt form.
#[derive(Clone)]
pub struct LoginForm {
    /// The email address of a user that attempts to login.
    pub email: String,
    /// The password of a user that attempts to login.
    pub password: String,
}

impl LoginForm {
    /// Creates a new [`LoginForm`].
    pub fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            password: password.into(),
        }
    }
}
