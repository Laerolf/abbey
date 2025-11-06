use std::fmt::Display;

use crate::shared::error::DomainError;

#[derive(Debug)]
pub enum MonasteryError {
    /// Failed to create a new [Monastery][`crate::features::monastery::domain::Monastery`].
    Creation,
}

impl std::error::Error for MonasteryError {}

impl DomainError for MonasteryError {
    /// Gets the locale code of a [`MonasteryError`].
    fn code(&self) -> &'static str {
        match self {
            Self::Creation => "error.monastery.creation",
        }
    }

    /// Gets the message of a [`MonasteryError`].
    fn message(&self) -> &'static str {
        match self {
            Self::Creation => "Failed to create a new monastery.",
        }
    }
}

impl Display for MonasteryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
