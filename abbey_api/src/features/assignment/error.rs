use std::fmt::Display;

use crate::shared::error::DomainError;

#[derive(Debug)]
pub enum AssignmentError {
    /// The [Actor][`crate::features::actor::domain::Actor`] is assigned to a [Process][`crate::features::process::domain::Process`].
    ActorAssigned,
}

impl std::error::Error for AssignmentError {}

impl DomainError for AssignmentError {
    /// Gets the locale code of a [`AssignmentError`].
    fn code(&self) -> &'static str {
        match self {
            Self::ActorAssigned => "error.assignment.actor_assigned",
        }
    }

    /// Gets the message of a [`AssignmentError`].
    fn message(&self) -> &'static str {
        match self {
            Self::ActorAssigned => "The provided actor is assigned to another process.",
        }
    }
}

impl Display for AssignmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
