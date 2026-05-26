use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum MonkErrorKind {
    /// Failed to get all Monks with the provided Monastery ID.
    GetAllByMonasteryId,
}

#[derive(Debug)]
pub enum ActorErrorKind {
    Monks(MonkErrorKind),
    /// Failed to create a new Actor.
    Creation,
    /// Failed to restore an Actor.
    Restore,
    /// Failed to update an Actor.
    Update,
    /// Failed to find an Actor with the provided ID.
    FindById,
    /// Failed to find Actors assigned to processes with the provided IDs.
    FindByProcessIds,
    /// Failed to get an Actor with the provided ID.
    GetById,
    /// Failed to get Actors with the provided IDs.
    GetByIds,
    /// Failed to get Actors with the provided CyclicProcess IDs.
    GetByCyclicProcessIds,
    /// The Actor is not available.
    Assigned,
    /// The Actor has no Skills.
    NoSkills,
    /// Failed to get CyclicProcesses.
    GetProcesses,
    /// Failed to get the Skill assignments for the provided Actor IDs.
    GetSkillAssignmentsByIds,
    /// The Actor has not been persisted yet.
    NotPersistedYet,
    Unknown,
}

impl DomainErrorKind for ActorErrorKind {
    /// Gets the locale code of the [`ActorErrorKind`].
    fn code(&self) -> String {
        match self {
            Self::Monks(error) => match error {
                MonkErrorKind::GetAllByMonasteryId => {
                    "error.actor.monks.get_all_by_monastery_id".to_string()
                }
            },
            Self::Creation => "error.actor.creation".to_string(),
            Self::Restore => "error.actor.restore".to_string(),
            Self::Update => "error.actor.update".to_string(),
            Self::FindById => "error.actor.find_by_id".to_string(),
            Self::FindByProcessIds => "error.actor.find_by_process_ids".to_string(),
            Self::GetById => "error.actor.find_by_id".to_string(),
            Self::GetByIds => "error.actor.find_by_ids".to_string(),
            Self::GetByCyclicProcessIds => "error.actor.find_by_cyclic_process_ids".to_string(),
            Self::Assigned => "error.actor.assigned".to_string(),
            Self::NoSkills => "error.actor.no_skills".to_string(),
            Self::GetProcesses => "error.actor.get_processes".to_string(),
            Self::GetSkillAssignmentsByIds => {
                "error.actor.get_skill_assignments_by_ids".to_string()
            }
            Self::NotPersistedYet => "error.actor.not_persisted_yet".to_string(),
            Self::Unknown => "error.actor.unknown".to_string(),
        }
    }

    /// Gets the message of the [`ActorErrorKind`].
    fn message(&self) -> String {
        match self {
            Self::Monks(error) => match error {
                MonkErrorKind::GetAllByMonasteryId => {
                    "Failed to get all Monks with the provided Monastery ID.".to_string()
                }
            },
            Self::Creation => "Failed to create a new Actor.".to_string(),
            Self::Restore => "Failed to create an Actor.".to_string(),
            Self::Update => "Failed to update an Actor.".to_string(),
            Self::FindById => "Failed to find an Actor with the provided ID.".to_string(),
            Self::FindByProcessIds => {
                "Failed to find Actors assigned to processes with the provided IDs.".to_string()
            }
            Self::GetById => "Failed to get an Actor with the provided ID.".to_string(),
            Self::GetByIds => "Failed to find Actors with the provided IDs.".to_string(),
            Self::GetByCyclicProcessIds => {
                "Failed to find Actors with the provided CyclicProcess IDs.".to_string()
            }
            Self::Assigned => "An Actor can only be assigned to one Process.".to_string(),
            Self::NoSkills => "The Actor has no Skills.".to_string(),
            Self::GetProcesses => "Failed to get CyclicProcesses.".to_string(),
            Self::GetSkillAssignmentsByIds => {
                "Failed to get the Skill assignments for the provided Actor IDs.".to_string()
            }
            Self::NotPersistedYet => "The Actor has not been persisted yet.".to_string(),
            Self::Unknown => "An unknown error occurred.".to_string(),
        }
    }

    fn http_status(&self) -> StatusCode {
        match self {
            Self::FindById => StatusCode::NOT_FOUND,
            Self::GetById => StatusCode::NOT_FOUND,
            Self::GetByIds => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn unknown() -> Self {
        Self::Unknown
    }
}

impl Display for ActorErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
