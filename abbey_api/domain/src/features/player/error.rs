use std::fmt::Display;

use crate::shared::error::DomainError;

#[derive(Debug)]
pub enum PlayerError {
    /// Failed to create a new [Player][`crate::features::player::domain::Player`].
    Creation,
}

impl std::error::Error for PlayerError {}

impl DomainError for PlayerError {
    /// Gets the locale code of a [`PlayerError`].
    fn code(&self) -> &'static str {
        match self {
            Self::Creation => "error.player.creation",
        }
    }

    /// Gets the message of a [`PlayerError`].
    fn message(&self) -> &'static str {
        match self {
            Self::Creation => "Failed to create a new player.",
        }
    }
}

impl Display for PlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
