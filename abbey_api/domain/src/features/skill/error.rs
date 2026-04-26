use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum SkillErrorKind {
    /// Failed to create a new [`Skill`][`super::domain::Skill`].
    Creation,
    /// Failed to find a Skill with the provided name.
    FindByName,
    /// Failed to find Skills with the provided names.
    FindByNames,
    Unknown,
}

impl DomainErrorKind for SkillErrorKind {
    /// Gets the locale code of the [`SkillError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.skill.creation".to_string(),
            Self::FindByName => "error.skill.find_by_name".to_string(),
            Self::FindByNames => "error.skill.find_by_names".to_string(),
            Self::Unknown => "error.skill.unknown".to_string(),
        }
    }

    /// Gets the message of the [`SkillError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new Skill.".to_string(),
            Self::FindByName => "Failed to find a Skill with the provided name.".to_string(),
            Self::FindByNames => "Failed to find a Skills with the provided names.".to_string(),
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
