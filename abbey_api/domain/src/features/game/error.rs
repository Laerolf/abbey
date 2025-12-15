use std::fmt::Display;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum GameErrorKind {
    /// Failed to create a new [Game][`crate::features::game::domain::Game`].
    Creation,
}

impl DomainErrorKind for GameErrorKind {
    /// Gets the locale code of a [`GameError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.game.creation".to_string(),
        }
    }

    /// Gets the message of a [`GameError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new game.".to_string(),
        }
    }
}

impl Display for GameErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
