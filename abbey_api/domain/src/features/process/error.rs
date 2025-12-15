use std::fmt::Display;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum ProcessErrorKind {
    /// Failed to create a new [Process][`super::domain::Process`].
    Creation,
    /// The [Process][`super::domain::Process`] is not new.
    NotNew,
    /// The [Process][`super::domain::Process`] is not in progress.
    NotInProgress,
    /// The [Process][`super::domain::Process`] is not paused.
    NotPaused,
    /// The [Process][`super::domain::Process`] is not complete.
    NotComplete,
    /// The [Process][`super::domain::Process`] has no assigned [People][`crate::features::actor::domain::person`].
    NoAssignedPeople,
}

impl DomainErrorKind for ProcessErrorKind {
    /// Gets the locale code of the [`ProcessError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.process.creation".to_string(),
            Self::NotNew => "error.process.not_new".to_string(),
            Self::NotInProgress => "error.process.not_in_progress".to_string(),
            Self::NotPaused => "error.process.not_paused".to_string(),
            Self::NotComplete => "error.process.not_complete".to_string(),
            Self::NoAssignedPeople => "error.process.no_assigned_people".to_string(),
        }
    }

    /// Gets the message of the [`ProcessError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new process.".to_string(),
            Self::NotNew => "The process is not new.".to_string(),
            Self::NotInProgress => "The process is not in progress.".to_string(),
            Self::NotPaused => "The process is not paused.".to_string(),
            Self::NotComplete => "The process is not complete yet.".to_string(),
            Self::NoAssignedPeople => "A process needs assigned people to be run.".to_string(),
        }
    }
}

impl Display for ProcessErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
