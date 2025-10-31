use std::fmt::Display;

use crate::shared::error::DomainError;

#[derive(Debug)]
pub enum ActorError {
    /// The [`crate::features::actor::domain::Actor`] is not available.
    Assigned,
}

impl std::error::Error for ActorError {}

impl DomainError for ActorError {
    /// Gets the locale code of the [`ActorError`].
    fn code(&self) -> &'static str {
        match self {
            Self::Assigned => "error.actor.assigned",
        }
    }

    /// Gets the message of the [`ActorError`].
    fn message(&self) -> &'static str {
        match self {
            Self::Assigned => "An actor can only be assigned to one process.",
        }
    }
}

impl Display for ActorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
