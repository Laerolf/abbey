use std::fmt::Display;

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
    /// Failed to find a [User][`super::domain::User`] by its email.
    FindByEmail,
}

impl DomainErrorKind for UserErrorKind {
    /// Gets the locale code of this [UserError].
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
            Self::FindByEmail => "error.user.find_by_email".to_string(),
        }
    }

    /// Gets the message of this [UserError].
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
            Self::FindByEmail => "Failed to find a user by its email.".to_string(),
        }
    }
}

impl Display for UserErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
