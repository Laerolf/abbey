use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum PlayerErrorKind {
    /// Failed to create a new [Player][`crate::features::player::domain::Player`].
    Creation,
    /// Failed to find a [Player][`crate::features::player::domain::Player`] by its ID.
    FindById,
    /// Failed to get a [Player][`crate::features::player::domain::Player`] by its ID.
    GetById,
    /// Failed to update a [Player][`crate::features::player::domain::Player`].
    Update,
    Unknown,
}

impl DomainErrorKind for PlayerErrorKind {
    /// Gets the locale code of a [`PlayerError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.player.creation".to_string(),
            Self::FindById => "error.player.find_by_id".to_string(),
            Self::GetById => "error.player.get_by_id".to_string(),
            Self::Update => "error.player.update".to_string(),
            Self::Unknown => "error.player.unknown".to_string(),
        }
    }

    /// Gets the message of a [`PlayerError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new player.".to_string(),
            Self::FindById => "Failed to find a player by its ID.".to_string(),
            Self::GetById => "Failed to get a player with the provided ID.".to_string(),
            Self::Update => "Failed to update a player.".to_string(),
            Self::Unknown => "An unknown error occurred.".to_string(),
        }
    }

    fn http_status(&self) -> StatusCode {
        match self {
            Self::FindById => StatusCode::NOT_FOUND,
            Self::GetById => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn unknown() -> Self {
        Self::Unknown
    }
}

impl Display for PlayerErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
