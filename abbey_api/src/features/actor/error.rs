use std::fmt::Display;

use crate::shared::error::DomainError;

#[derive(Debug)]
pub enum ActorError {
    Assigned,
}

impl std::error::Error for ActorError {}

impl DomainError for ActorError {
    fn code(&self) -> &'static str {
        match self {
            Self::Assigned => "error.actor.assigned",
        }
    }

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
