use std::fmt::Display;

use crate::shared::error::DomainError;

#[derive(Debug)]
pub enum ProcessError {
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

impl std::error::Error for ProcessError {}

impl DomainError for ProcessError {
    /// Gets the locale code of the [`ProcessError`].
    fn code(&self) -> &'static str {
        match self {
            Self::NotNew => "error.process.not_new",
            Self::NotInProgress => "error.process.not_in_progress",
            Self::NotPaused => "error.process.not_paused",
            Self::NotComplete => "error.process.not_complete",
            Self::NoAssignedPeople => "error.process.no_assigned_people",
        }
    }

    /// Gets the message of the [`ProcessError`].
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
