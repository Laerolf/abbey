use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum ProcessErrorKind {
    /// Failed to create a new [Process][`super::domain::Process`].
    Creation,
    /// Failed to update a Process.
    Update,
    /// Failed to start a Process.
    Start,
    /// Failed to pause a Process.
    Pause,
    /// Failed to get all the resources of a Process.
    GetAllResources,
    /// Failed to find a Process with the provided ID.
    FindById,
    /// Failed to get a Process with the provided ID.
    GetById,
    /// Failed to get the Processes with the provided IDs.
    GetByIds,
    /// Failed to find a Process with the provided ID and Game ID.
    FindByIdForGame,
    /// Failed to find the Actors of a Process.
    FindActors,
    /// Failed to get a Process.
    NotFound,
    /// The process has no input resources.
    NoInputResources,
    /// The process has no output resources.
    NoOutputResources,
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
    /// The Process has not been persisted yet.
    NotPersistedYet,
    Unknown,
}

impl DomainErrorKind for ProcessErrorKind {
    /// Gets the locale code of the [`ProcessErrorKind`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.process.creation".to_string(),
            Self::Update => "error.process.update".to_string(),
            Self::Start => "error.process.start".to_string(),
            Self::Pause => "error.process.pause".to_string(),
            Self::GetAllResources => "error.process.get_all_resources".to_string(),
            Self::FindById => "error.process.find_by_id".to_string(),
            Self::GetById => "error.process.get_by_id".to_string(),
            Self::GetByIds => "error.process.get_by_ids".to_string(),
            Self::FindByIdForGame => "error.process.find_by_id_for_game".to_string(),
            Self::FindActors => "error.process.find_actors".to_string(),
            Self::NotFound => "error.process.not_found".to_string(),
            Self::NoInputResources => "error.process.no_input_resources".to_string(),
            Self::NoOutputResources => "error.process.no_output_resources".to_string(),
            Self::NotNew => "error.process.not_new".to_string(),
            Self::NotInProgress => "error.process.not_in_progress".to_string(),
            Self::NotPaused => "error.process.not_paused".to_string(),
            Self::NotComplete => "error.process.not_complete".to_string(),
            Self::NoAssignedPeople => "error.process.no_assigned_people".to_string(),
            Self::NotPersistedYet => "error.process.not_persisted_yet".to_string(),
            Self::Unknown => "error.shared.unknown".to_string(),
        }
    }

    /// Gets the message of the [`ProcessErrorKind`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new process.".to_string(),
            Self::Update => "Failed to update a process.".to_string(),
            Self::Start => "Failed to start a process.".to_string(),
            Self::Pause => "Failed to pause a process.".to_string(),
            Self::GetAllResources => "Failed to get all the resources of a process.".to_string(),
            Self::FindById => "Failed to find a process with the provided ID.".to_string(),
            Self::GetById => "Failed to get a process with the provided ID.".to_string(),
            Self::GetByIds => "Failed to get the processes with the provided IDs.".to_string(),
            Self::FindByIdForGame => {
                "Failed to find a process with the provided ID and Game ID.".to_string()
            }
            Self::FindActors => "Failed to find the actors of a process.".to_string(),
            Self::NotFound => "Failed to get a process.".to_string(),
            Self::NoInputResources => "The process has no input resources.".to_string(),
            Self::NoOutputResources => "The process has no output resources.".to_string(),
            Self::NotNew => "The process is not new.".to_string(),
            Self::NotInProgress => "The process is not in progress.".to_string(),
            Self::NotPaused => "The process is not paused.".to_string(),
            Self::NotComplete => "The process is not complete yet.".to_string(),
            Self::NoAssignedPeople => "A process needs assigned people to be run.".to_string(),
            Self::NotPersistedYet => "The Process has not been persisted yet.".to_string(),
            Self::Unknown => "An unknown error occurred.".to_string(),
        }
    }

    /// Gets the [HTTP status code][StatusCode] of the [`ProcessErrorKind`].
    fn http_status(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn unknown() -> Self {
        Self::Unknown
    }
}

impl Display for ProcessErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
