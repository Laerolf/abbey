use axum::{
    Json,
    http::{StatusCode, header},
    response::IntoResponse,
};
use cookie::Cookie;
use serde::{Deserialize, Serialize};
use tower_cookies::cookie;
use utoipa::ToSchema;

use crate::features::auth::domain::{AuthenticationTokens, refresh_token::RefreshToken};

/// Represents the payload used to register a new user.
#[derive(Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RegisterUserRequest {
    /// The email address of the user to register.
    #[schema(example = "ozzy@in.heaven")]
    pub email: Option<String>,
    /// The password of the user to register.
    #[schema(example = "live")]
    pub password: Option<String>,
    /// The confirmed password of the user to register.
    #[schema(example = "live")]
    pub confirmed_password: Option<String>,
}

/// Represents the payload used to login a user.
#[derive(Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct LoginUserRequest {
    /// The email address of the user to login with.
    #[schema(example = "ozzy@in.heaven")]
    pub email: Option<String>,
    /// The password of the user to login with.
    #[schema(example = "live")]
    pub password: Option<String>,
}

/// Represents the response used when a new user was registered.
#[derive(Serialize, ToSchema)]
pub struct RegisterUserResponse {
    /// The ID of the registered user.
    #[schema(example = 666)]
    pub id: i32,
    /// The email address of the registered user.
    #[schema(example = "ozzy@in.heaven")]
    pub email: String,
}

impl RegisterUserResponse {
    /// Creates a new [RegisterUserResponse].
    pub fn new(id: i32, email: impl Into<String>) -> RegisterUserResponse {
        RegisterUserResponse {
            id,
            email: email.into(),
        }
    }
}

/// Represents the response used when a user was logged-in.
#[derive(Serialize, ToSchema)]
pub struct LoginUserResponse {
    /// The session token for the logged-in user.
    #[schema(example = "i66n5")]
    pub session_token: String,
    /// The refresh token for the logged-in user.
    #[serde(skip)]
    pub refresh_token: RefreshToken,
}

impl LoginUserResponse {
    /// Creates a new [LoginUserResponse].
    pub fn new(session_token: impl Into<String>, refresh_token: RefreshToken) -> Self {
        Self {
            session_token: session_token.into(),
            refresh_token,
        }
    }
}

impl IntoResponse for LoginUserResponse {
    fn into_response(self) -> axum::response::Response {
        let refresh_token_cookie = Cookie::build((
            AuthenticationTokens::RefreshToken.cookie_name(),
            self.refresh_token.value().to_string(),
        ))
        .path("/")
        .max_age(self.refresh_token.lifespan())
        .same_site(cookie::SameSite::Strict)
        .http_only(true)
        .secure(true)
        .build();

        (
            StatusCode::OK,
            [(header::SET_COOKIE, refresh_token_cookie.to_string())],
            Json(self),
        )
            .into_response()
    }
}

/// Represents the response used when a user refresh its session token.
#[derive(Serialize, ToSchema)]
pub struct RefreshUserResponse {
    /// The session token.
    #[schema(example = "666999")]
    pub session_token: String,
    /// The refresh token.
    #[serde(skip)]
    pub refresh_token: RefreshToken,
}

impl RefreshUserResponse {
    /// Creates a new [RefreshUserResponse].
    pub fn new(session_token: impl Into<String>, refresh_token: RefreshToken) -> Self {
        Self {
            session_token: session_token.into(),
            refresh_token,
        }
    }
}

impl IntoResponse for RefreshUserResponse {
    fn into_response(self) -> axum::response::Response {
        let refresh_token_cookie = Cookie::build((
            AuthenticationTokens::RefreshToken.cookie_name(),
            self.refresh_token.value().to_string(),
        ))
        .path("/")
        .max_age(self.refresh_token.lifespan())
        .same_site(cookie::SameSite::Strict)
        .http_only(true)
        .secure(true)
        .build();

        (
            StatusCode::OK,
            [(header::SET_COOKIE, refresh_token_cookie.to_string())],
            Json(self),
        )
            .into_response()
    }
}
