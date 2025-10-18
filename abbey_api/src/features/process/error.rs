use std::fmt::Display;

use crate::shared::error::DomainError;

#[derive(Debug)]
pub enum ProcessError {
    NotNew,
    NotInProgress,
    NotPaused,
    NotComplete,
    NoAssignedPeople,
}

impl DomainError for ProcessError {
    fn code(&self) -> &'static str {
        match self {
            Self::NotNew => "error.process.not_new",
            Self::NotInProgress => "error.process.not_in_progress",
            Self::NotPaused => "error.process.not_paused",
            Self::NotComplete => "error.process.not_complete",
            Self::NoAssignedPeople => "error.process.no_assigned_people",
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Self::NotNew => "The process is not new.",
            Self::NotInProgress => "The process is not in progress.",
            Self::NotPaused => "The process is not paused.",
            Self::NotComplete => "The process is not complete yet.",
            Self::NoAssignedPeople => "A process needs assigned people to be run.",
        }
    }
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
