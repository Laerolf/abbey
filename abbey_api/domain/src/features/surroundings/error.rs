use std::fmt::Display;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum SurroundingsErrorKind {
    /// Failed to create a new [Surroundings][`crate::features::surroundings::domain::Surroundings`].
    Creation,
}

impl DomainErrorKind for SurroundingsErrorKind {
    /// Gets the locale code of a [`SurroundingsError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.surroundings.creation".to_string(),
        }
    }

    /// Gets the message of a [`SurroundingsError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create new surroundings.".to_string(),
        }
    }
}

impl Display for SurroundingsErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
