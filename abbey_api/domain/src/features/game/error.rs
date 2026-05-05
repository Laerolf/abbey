use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum GameErrorKind {
    /// Failed to create a new [Game][`crate::features::game::domain::Game`].
    Creation,
    /// Failed to find the Player of a Game.
    PlayerNotFound,
    /// Failed to find the Monastery of a Game.
    MonasteryNotFound,
    /// Failed to find the Surroundings of a Game.
    SurroundingsNotFound,
    /// Failed to find a [Game][`crate::features::game::domain::Game`] by its ID.
    FindById,
    /// Failed to get a [Game][`crate::features::game::domain::Game`] by its ID.
    GetById,
    /// The Game has not been persisted yet.
    NotPersistedYet,
    Unknown,
}

impl DomainErrorKind for GameErrorKind {
    /// Gets the locale code of a [`GameError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.game.creation".to_string(),
            Self::PlayerNotFound => "error.game.player_not_found".to_string(),
            Self::MonasteryNotFound => "error.game.monastery_not_found".to_string(),
            Self::SurroundingsNotFound => "error.game.surroundings_not_found".to_string(),
            Self::FindById => "error.game.find_by_id".to_string(),
            Self::GetById => "error.game.get_by_id".to_string(),
            Self::NotPersistedYet => "error.game.not_persisted_yet".to_string(),
            Self::Unknown => "error.game.unknown".to_string(),
        }
    }

    /// Gets the message of a [`GameError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new game.".to_string(),
            Self::PlayerNotFound => "Failed to find the player of a game.".to_string(),
            Self::MonasteryNotFound => "Failed to find the monastery of a game.".to_string(),
            Self::SurroundingsNotFound => "Failed to find the surroundings of a game.".to_string(),
            Self::FindById => "Failed to find a game by its ID.".to_string(),
            Self::GetById => "Failed to get a game by its ID.".to_string(),
            Self::NotPersistedYet => "The Game has not been persisted yet.".to_string(),
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

impl Display for GameErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
