use time::{Duration, OffsetDateTime};

use crate::features::auth::domain::refresh_token::RefreshTokenValue;

const REFRESH_TOKEN_LIFESPAN: Duration = Duration::weeks(2);

/// Represents a user account registration form.
#[derive(Clone)]
pub struct UserRegistrationForm {
    /// The email address of the new user account to create.
    pub email: String,
    /// The password of the new user account to create.
    pub password: String,
}

impl UserRegistrationForm {
    /// Creates a new [`UserRegistrationForm`].
    pub fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            password: password.into(),
        }
    }
}

/// Represents a RefreshToken blueprint.
pub struct RefreshTokenBlueprint {
    /// The user ID of the RefreshToken to create.
    pub user_id: i32,
    /// The value of the RefreshToken to create.
    pub value: String,
    /// The creation time of the RefreshToken to create.
    pub created_at: OffsetDateTime,
    /// The expiration time of the RefreshToken to create.
    pub expires_at: OffsetDateTime,
}

impl RefreshTokenBlueprint {
    /// Creates a new [`RefreshTokenBlueprint`].
    pub fn new(user_id: &i32) -> Self {
        let created_at = OffsetDateTime::now_utc();
        let expires_at = created_at.saturating_add(REFRESH_TOKEN_LIFESPAN);

        Self {
            user_id: *user_id,
            value: RefreshTokenValue::default().to_string().to_owned(),
            created_at,
            expires_at,
        }
    }
}

/// Represents a user login attempt form.
#[derive(Clone)]
pub struct UserLoginForm {
    /// The email address of a user that attempts to login.
    pub email: String,
    /// The password of a user that attempts to login.
    pub password: String,
}

impl UserLoginForm {
    /// Creates a new [`UserLoginForm`].
    pub fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            password: password.into(),
        }
    }
}
