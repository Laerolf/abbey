use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum SkillErrorKind {
    /// Failed to create a new Skill.
    Creation,
    /// Failed to get all Skills.
    GetAll,
    /// Failed to get Skills with the provided IDs.
    GetByIds,
    /// Failed to find a Skill with the provided name.
    FindByName,
    /// Failed to find Skills with the provided names.
    FindByNames,
    /// The Skill has not been persisted yet.
    NotPersistedYet,
    Unknown,
}

impl DomainErrorKind for SkillErrorKind {
    /// Gets the locale code of the [`SkillErrorKind`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.skill.creation".to_string(),
            Self::GetAll => "error.skill.get_all".to_string(),
            Self::GetByIds => "error.skill.get_by_ids".to_string(),
            Self::FindByName => "error.skill.find_by_name".to_string(),
            Self::FindByNames => "error.skill.find_by_names".to_string(),
            Self::NotPersistedYet => "error.skill.not_persisted_yet".to_string(),
            Self::Unknown => "error.skill.unknown".to_string(),
        }
    }

    /// Gets the message of the [`SkillErrorKind`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new Skill.".to_string(),
            Self::GetAll => "Failed to get all Skills.".to_string(),
            Self::GetByIds => "Failed to get Skills with the provided IDs.".to_string(),
            Self::FindByName => "Failed to find a Skill with the provided name.".to_string(),
            Self::FindByNames => "Failed to find a Skills with the provided names.".to_string(),
            Self::NotPersistedYet => "The Skill has not been persisted yet.".to_string(),
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

impl Display for SkillErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
