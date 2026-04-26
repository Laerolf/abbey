use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum UserCreationErrorKind {
    EmailRequired,
    EmailInvalid(String),
    EmailAlreadyExists(String),
    AssignGame,
    /// Failed to create a [Game][`crate::features::game::domain::Game`] for a new [User][`super::domain::User`].
    CreateGame,
    Unknown,
}

#[derive(Debug)]
pub enum UserErrorKind {
    /// Failed to create a new [User][`super::domain::User`].
    Creation(UserCreationErrorKind),
    /// A User requires Games.
    MissingGames,
    /// Failed to persist a new [User][`super::domain::User`].
    Insert,
    /// Failed to update a [User][`super::domain::User`].
    Update,
    /// Failed to get all Games for a [User][`super::domain::User`] by its ID.
    GetAllGamesByUserId,
    /// Failed to find a [User][`super::domain::User`] by its ID.
    FindById,
    /// Failed to get a [User][`super::domain::User`] by its ID.
    GetById,
    /// Failed to find a [User][`super::domain::User`] by its email.
    FindByEmail,
    Unknown,
}

impl DomainErrorKind for UserErrorKind {
    /// Gets the locale code of this [UserErrorKind].
    fn code(&self) -> String {
        match self {
            Self::Creation(err) => match err {
                UserCreationErrorKind::EmailRequired => {
                    "error.user.creation.email_required".to_string()
                }
                UserCreationErrorKind::EmailInvalid(_) => {
                    "error.user.creation.email_invalid".to_string()
                }
                UserCreationErrorKind::EmailAlreadyExists(_) => {
                    "error.user.creation.email_exists".to_string()
                }
                UserCreationErrorKind::CreateGame => "error.user.creation.create_game".to_string(),
                UserCreationErrorKind::AssignGame => "error.user.creation.assign_game".to_string(),
                UserCreationErrorKind::Unknown => "error.user.creation.unknown".to_string(),
            },
            Self::MissingGames => "error.user.missing_games".to_string(),
            Self::Insert => "error.user.insert".to_string(),
            Self::Update => "error.user.update".to_string(),
            Self::GetAllGamesByUserId => "error.user.get_all_games_by_user_id".to_string(),
            Self::FindById => "error.user.find_by_id".to_string(),
            Self::GetById => "error.user.get_by_id".to_string(),
            Self::FindByEmail => "error.user.find_by_email".to_string(),
            Self::Unknown => "error.user.unknown".to_string(),
        }
    }

    /// Gets the message of this [UserErrorKind].
    fn message(&self) -> String {
        match self {
            Self::Creation(err) => match err {
                UserCreationErrorKind::EmailRequired => "Email is required.".to_string(),
                UserCreationErrorKind::EmailInvalid(email) => {
                    format!("Email '{}' is invalid.", email)
                }
                UserCreationErrorKind::EmailAlreadyExists(email) => {
                    format!("Email '{}' already exists", email)
                }
                UserCreationErrorKind::CreateGame => {
                    "Failed to create a new game for a new user.".to_string()
                }
                UserCreationErrorKind::AssignGame => {
                    "Failed to assign a game to a new user.".to_string()
                }
                UserCreationErrorKind::Unknown => "Unknown reason.".to_string(),
            },
            Self::MissingGames => "A User requires Games.".to_string(),
            Self::Insert => "Failed to persist a new user.".to_string(),
            Self::Update => "Failed to update a user.".to_string(),
            Self::GetAllGamesByUserId => "Failed to get all games for a user.".to_string(),
            Self::FindById => "Failed to find a user by its ID.".to_string(),
            Self::GetById => "Failed to get a user by its ID.".to_string(),
            Self::FindByEmail => "Failed to find a user by its email.".to_string(),
            Self::Unknown => "An unknown error occurred.".to_string(),
        }
    }

    fn http_status(&self) -> StatusCode {
        match self {
            Self::GetById => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn unknown() -> Self {
        Self::Unknown
    }
}

impl Display for UserErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
