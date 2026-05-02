use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum RegistrationErrorKind {
    EmailRequired,
    EmailInvalid,
    EmailAlreadyExists,
    PasswordRequired,
    ConfirmedPasswordRequired,
    InvalidConfirmedPassword,
    HashPassword,
    CreateUser,
}

#[derive(Debug)]
pub enum LoginErrorKind {
    EmailRequired,
    PasswordRequired,
    WrongCredentials,
    StartDatabaseTransaction,
    CreateSessionToken,
    CreateRefreshToken,
}

#[derive(Debug)]
pub enum RefreshErrorKind {
    StartDatabaseTransaction,
    RefreshTokenNotFound,
    UserNotFound,
    DeleteRefreshToken,
    CreateSessionToken,
    CreateRefreshToken,
    FindById,
}

#[derive(Debug)]
pub enum GetUserSessionErrorKind {
    UserNotFound,
    GameNotFound,
}

#[derive(Debug)]
pub enum AuthenticationErrorKind {
    /// Failed to register a new user.
    Registration(RegistrationErrorKind),
    /// Failed to login a user.
    Login(LoginErrorKind),
    /// Failed to refresh a user's credentials.
    Refresh(RefreshErrorKind),
    /// Failed authenticate a user.
    Authenticate,
    /// Failed to get a user session.
    GetUserSession(GetUserSessionErrorKind),
    Unknown,
}

impl DomainErrorKind for AuthenticationErrorKind {
    /// Gets the locale code of this [AuthenticationErrorKind].
    fn code(&self) -> String {
        match self {
            Self::Registration(error) => match error {
                RegistrationErrorKind::EmailRequired => {
                    "error.authentication.registration.email_required".to_string()
                }
                RegistrationErrorKind::EmailInvalid => {
                    "error.authentication.registration.email_invalid".to_string()
                }
                RegistrationErrorKind::EmailAlreadyExists => {
                    "error.authentication.registration.email_exists".to_string()
                }
                RegistrationErrorKind::PasswordRequired => {
                    "error.authentication.registration.password_required".to_string()
                }
                RegistrationErrorKind::ConfirmedPasswordRequired => {
                    "error.authentication.registration.confirmed_password_required".to_string()
                }
                RegistrationErrorKind::InvalidConfirmedPassword => {
                    "error.authentication.registration.invalid_confirmed_password".to_string()
                }
                RegistrationErrorKind::HashPassword => {
                    "error.authentication.registration.hash_password".to_string()
                }
                RegistrationErrorKind::CreateUser => {
                    "error.authentication.registration.create_user".to_string()
                }
            },
            Self::Login(error) => match error {
                LoginErrorKind::EmailRequired => {
                    "error.authentication.login.email_required".to_string()
                }
                LoginErrorKind::PasswordRequired => {
                    "error.authentication.login.password_required".to_string()
                }
                LoginErrorKind::WrongCredentials => {
                    "error.authentication.login.wrong_credentials".to_string()
                }
                LoginErrorKind::StartDatabaseTransaction => {
                    "error.authentication.login.start_database_transaction".to_string()
                }
                LoginErrorKind::CreateSessionToken => {
                    "error.authentication.login.create_session_token".to_string()
                }
                LoginErrorKind::CreateRefreshToken => {
                    "error.authentication.login.create_refresh_token".to_string()
                }
            },
            Self::Refresh(error) => match error {
                RefreshErrorKind::StartDatabaseTransaction => {
                    "error.authentication.refresh.start_database_transaction".to_string()
                }
                RefreshErrorKind::RefreshTokenNotFound => {
                    "error.authentication.refresh.refresh_token_not_found".to_string()
                }
                RefreshErrorKind::UserNotFound => {
                    "error.authentication.refresh.user_not_found".to_string()
                }
                RefreshErrorKind::DeleteRefreshToken => {
                    "error.authentication.refresh.delete_refresh_token".to_string()
                }
                RefreshErrorKind::CreateSessionToken => {
                    "error.authentication.refresh.create_session_token".to_string()
                }
                RefreshErrorKind::CreateRefreshToken => {
                    "error.authentication.refresh.create_refresh_token".to_string()
                }
                RefreshErrorKind::FindById => "error.authentication.refresh.find_by_id".to_string(),
            },
            AuthenticationErrorKind::Authenticate => {
                "error.authentication.authenticate".to_string()
            }
            Self::GetUserSession(error) => match error {
                GetUserSessionErrorKind::UserNotFound => {
                    "error.authentication.get_user_session.user_not_found".to_string()
                }
                GetUserSessionErrorKind::GameNotFound => {
                    "error.authentication.get_user_session.game_not_found".to_string()
                }
            },
            Self::Unknown => "error.authentication.unknown".to_string(),
        }
    }

    /// Gets the message of this [AuthenticationErrorKind].
    fn message(&self) -> String {
        match self {
            Self::Registration(err) => match err {
                RegistrationErrorKind::EmailRequired => "An email is required.".to_string(),
                RegistrationErrorKind::EmailInvalid => "The provided email is invalid.".to_string(),
                RegistrationErrorKind::EmailAlreadyExists => {
                    "The provided email already exists.".to_string()
                }
                RegistrationErrorKind::PasswordRequired => "A password is required.".to_string(),
                RegistrationErrorKind::ConfirmedPasswordRequired => {
                    "A confirming password is required.".to_string()
                }
                RegistrationErrorKind::InvalidConfirmedPassword => {
                    "The provided password has not been confirmed.".to_string()
                }
                RegistrationErrorKind::HashPassword => {
                    "Failed to hash a user registration password.".to_string()
                }
                RegistrationErrorKind::CreateUser => "Failed to create a user.".to_string(),
            },
            Self::Login(error) => match error {
                LoginErrorKind::EmailRequired => "An email is required.".to_string(),
                LoginErrorKind::PasswordRequired => "A password is required.".to_string(),
                LoginErrorKind::WrongCredentials => {
                    "The provided mail or password is incorrect.".to_string()
                }
                LoginErrorKind::StartDatabaseTransaction => {
                    "Failed to start a database transaction.".to_string()
                }
                LoginErrorKind::CreateSessionToken => {
                    "Failed to create a new session token.".to_string()
                }
                LoginErrorKind::CreateRefreshToken => {
                    "Failed to create a new refresh token.".to_string()
                }
            },
            Self::Refresh(error) => match error {
                RefreshErrorKind::StartDatabaseTransaction => {
                    "Failed to start a database transaction.".to_string()
                }
                RefreshErrorKind::RefreshTokenNotFound => {
                    "Failed to find the refresh token.".to_string()
                }
                RefreshErrorKind::UserNotFound => {
                    "Failed to find the user of a refresh token.".to_string()
                }
                RefreshErrorKind::DeleteRefreshToken => {
                    "Failed to delete a refresh token.".to_string()
                }
                RefreshErrorKind::CreateSessionToken => {
                    "Failed to create a new session token.".to_string()
                }
                RefreshErrorKind::CreateRefreshToken => {
                    "Failed to create a new refresh token.".to_string()
                }
                RefreshErrorKind::FindById => {
                    "Failed to find a refresh token with the provided ID.".to_string()
                }
            },
            AuthenticationErrorKind::Authenticate => "Failed to authenticate a user.".to_string(),
            Self::GetUserSession(error) => match error {
                GetUserSessionErrorKind::UserNotFound => {
                    "Failed to find the user of a user session.".to_string()
                }
                GetUserSessionErrorKind::GameNotFound => {
                    "Failed to find the game of a user session.".to_string()
                }
            },
            Self::Unknown => "An unknown error occurred.".to_string(),
        }
    }

    fn http_status(&self) -> StatusCode {
        match self {
            AuthenticationErrorKind::Registration(error) => match error {
                RegistrationErrorKind::EmailRequired => StatusCode::BAD_REQUEST,
                RegistrationErrorKind::PasswordRequired => StatusCode::BAD_REQUEST,
                RegistrationErrorKind::ConfirmedPasswordRequired => StatusCode::BAD_REQUEST,
                RegistrationErrorKind::InvalidConfirmedPassword => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AuthenticationErrorKind::Login(error) => match error {
                LoginErrorKind::EmailRequired => StatusCode::BAD_REQUEST,
                LoginErrorKind::PasswordRequired => StatusCode::BAD_REQUEST,
                LoginErrorKind::WrongCredentials => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AuthenticationErrorKind::Refresh(error) => match error {
                RefreshErrorKind::RefreshTokenNotFound => StatusCode::UNAUTHORIZED,
                RefreshErrorKind::UserNotFound => StatusCode::UNAUTHORIZED,
                RefreshErrorKind::FindById => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AuthenticationErrorKind::Authenticate => StatusCode::UNAUTHORIZED,
            AuthenticationErrorKind::GetUserSession(_error) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn unknown() -> Self {
        Self::Unknown
    }
}

impl Display for AuthenticationErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
