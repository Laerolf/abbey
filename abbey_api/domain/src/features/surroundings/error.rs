use std::fmt::Display;

use crate::shared::error::DomainError;

#[derive(Debug)]
pub enum SurroundingsError {
    /// Failed to create a new [Surroundings][`crate::features::surroundings::domain::Surroundings`].
    Creation,
}

impl std::error::Error for SurroundingsError {}

impl DomainError for SurroundingsError {
    /// Gets the locale code of a [`SurroundingsError`].
    fn code(&self) -> &'static str {
        match self {
            Self::Creation => "error.surroundings.creation",
        }
    }

    /// Gets the message of a [`SurroundingsError`].
    fn message(&self) -> &'static str {
        match self {
            Self::Creation => "Failed to create new surroundings.",
        }
    }
}

impl Display for SurroundingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
