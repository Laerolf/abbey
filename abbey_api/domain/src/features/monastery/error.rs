use std::fmt::Display;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum MonasteryErrorKind {
    /// Failed to create a new [Monastery][`crate::features::monastery::domain::Monastery`].
    Creation,
}

impl DomainErrorKind for MonasteryErrorKind {
    /// Gets the locale code of a [`MonasteryError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.monastery.creation".to_string(),
        }
    }

    /// Gets the message of a [`MonasteryError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new monastery.".to_string(),
        }
    }
}

impl Display for MonasteryErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
