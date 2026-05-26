use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum SurroundingsErrorKind {
    /// Failed to create a new [Surroundings][`crate::features::surroundings::domain::Surroundings`].
    Creation,
    /// This Surroundings have no Sources.
    MissingSources,
    /// Failed to get all Sources of a Surroundings.
    GetAllSources,
    /// Failed to find a Surroundings with the provided ID.
    FindById,
    /// Failed to get a Surroundings with the provided ID.
    GetById,
    /// The Surroundings has not been persisted yet.
    NotPersistedYet,
    Unknown,
}

impl DomainErrorKind for SurroundingsErrorKind {
    /// Gets the locale code of a [`SurroundingsError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.surroundings.creation".to_string(),
            Self::MissingSources => "error.surroundings.missing_sources".to_string(),
            Self::GetAllSources => "error.surroundings.get_all_sources".to_string(),
            Self::FindById => "error.surroundings.find_by_id".to_string(),
            Self::GetById => "error.surroundings.get_by_id".to_string(),
            Self::NotPersistedYet => "error.surroundings.not_persisted_yet".to_string(),
            Self::Unknown => "error.surroundings.unknown".to_string(),
        }
    }

    /// Gets the message of a [`SurroundingsError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create new surroundings.".to_string(),
            Self::MissingSources => "This Surroundings have no Sources.".to_string(),
            Self::GetAllSources => "Failed to get all Sources of a Surroundings.".to_string(),
            Self::FindById => "Failed to find a Surroundings with the provided ID.".to_string(),
            Self::GetById => "Failed to get a Surroundings with the provided ID.".to_string(),
            Self::NotPersistedYet => "The Surroundings has not been persisted yet.".to_string(),
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

impl Display for SurroundingsErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
