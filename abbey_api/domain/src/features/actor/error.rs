use std::fmt::Display;

use crate::shared::error::DomainError;

#[derive(Debug)]
pub enum ActorError {
    /// Failed to create a new [`Actor`][`super::domain::Actor`].
    Creation,
    /// The [`Actor`][`super::domain::Actor`] is not available.
    Assigned,
}

impl std::error::Error for ActorError {}

impl DomainError for ActorError {
    /// Gets the locale code of the [`ActorError`].
    fn code(&self) -> &'static str {
        match self {
            Self::Creation => "error.actor.creation",
            Self::Assigned => "error.actor.assigned",
        }
    }

    /// Gets the message of the [`ActorError`].
    fn message(&self) -> &'static str {
        match self {
            Self::Creation => "Failed to create a new Actor.",
            Self::Assigned => "An Actor can only be assigned to one Process.",
        }
    }
}

impl Display for ActorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
