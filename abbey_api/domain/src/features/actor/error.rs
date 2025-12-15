use std::fmt::Display;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum ActorErrorKind {
    /// Failed to create a new [`Actor`][`super::domain::Actor`].
    Creation,
    /// The [`Actor`][`super::domain::Actor`] is not available.
    Assigned,
}

impl DomainErrorKind for ActorErrorKind {
    /// Gets the locale code of the [`ActorError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.actor.creation".to_string(),
            Self::Assigned => "error.actor.assigned".to_string(),
        }
    }

    /// Gets the message of the [`ActorError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new Actor.".to_string(),
            Self::Assigned => "An Actor can only be assigned to one Process.".to_string(),
        }
    }
}

impl Display for ActorErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
