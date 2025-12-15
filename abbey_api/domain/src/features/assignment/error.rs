use std::fmt::Display;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum AssignmentErrorKind {
    /// The [Actor][`crate::features::actor::domain::Actor`] is assigned to a [Process][`crate::features::process::domain::Process`].
    ActorAssigned,
}

impl DomainErrorKind for AssignmentErrorKind {
    /// Gets the locale code of a [`AssignmentError`].
    fn code(&self) -> String {
        match self {
            Self::ActorAssigned => "error.assignment.actor_assigned".to_string(),
        }
    }

    /// Gets the message of a [`AssignmentError`].
    fn message(&self) -> String {
        match self {
            Self::ActorAssigned => "The provided actor is assigned to another process.".to_string(),
        }
    }
}

impl Display for AssignmentErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
