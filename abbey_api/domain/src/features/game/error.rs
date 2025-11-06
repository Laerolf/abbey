use std::fmt::Display;

use crate::shared::error::DomainError;

#[derive(Debug)]
pub enum GameError {
    /// Failed to create a new [Game][`crate::features::game::domain::Game`].
    Creation,
}

impl std::error::Error for GameError {}

impl DomainError for GameError {
    /// Gets the locale code of a [`GameError`].
    fn code(&self) -> &'static str {
        match self {
            Self::Creation => "error.game.creation",
        }
    }

    /// Gets the message of a [`GameError`].
    fn message(&self) -> &'static str {
        match self {
            Self::Creation => "Failed to create a new game.",
        }
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
