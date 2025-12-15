use std::fmt::Display;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum PlayerErrorKind {
    /// Failed to create a new [Player][`crate::features::player::domain::Player`].
    Creation,
    /// Failed find a [Player][`crate::features::player::domain::Player`] by its ID.
    FindById,
}

impl DomainErrorKind for PlayerErrorKind {
    /// Gets the locale code of a [`PlayerError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.player.creation".to_string(),
            Self::FindById => "error.player.findById".to_string(),
        }
    }

    /// Gets the message of a [`PlayerError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new player.".to_string(),
            Self::FindById => "Failed to find a player by its ID.".to_string(),
        }
    }
}

impl Display for PlayerErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
