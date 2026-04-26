use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum AssignmentErrorKind {
    /// The [Actor][`crate::features::actor::domain::Actor`] is assigned to a [Process][`crate::features::process::domain::Process`].
    ActorAssigned,
    /// Failed to find the Actor to assign.
    ActorNotFound,
    /// Failed to find the Process to assign.
    ProcessNotFound,
    /// Failed to assign an Actor to a Process.
    ActorAssignment,
    /// Failed to assign a Process to an Actor.
    ProcessAssignment,
    Unknown,
}

impl DomainErrorKind for AssignmentErrorKind {
    /// Gets the locale code of a [`AssignmentErrorKind`].
    fn code(&self) -> String {
        match self {
            Self::ActorAssigned => "error.assignment.actor_assigned".to_string(),
            Self::ActorNotFound => "error.assignment.actor_not_found".to_string(),
            Self::ProcessNotFound => "error.assignment.process_not_found".to_string(),
            Self::ActorAssignment => "error.assignment.actor_assignment".to_string(),
            Self::ProcessAssignment => "error.assignment.process_assignment".to_string(),
            Self::Unknown => "error.assignment.unknown".to_string(),
        }
    }

    /// Gets the message of a [`AssignmentErrorKind`].
    fn message(&self) -> String {
        match self {
            Self::ActorAssigned => "The provided actor is assigned to another process.".to_string(),
            Self::ActorNotFound => "Failed to find the actor to assign.".to_string(),
            Self::ProcessNotFound => "Failed to find the process to assign.".to_string(),
            Self::ActorAssignment => "Failed to assign an actor to a process.".to_string(),
            Self::ProcessAssignment => "Failed to assign a process to an actor.".to_string(),
            Self::Unknown => "An unknown error occurred.".to_string(),
        }
    }

    fn http_status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn unknown() -> Self {
        Self::Unknown
    }
}

impl Display for AssignmentErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
