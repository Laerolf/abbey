use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum SourceErrorKind {
    /// Failed to create a new [Source][`super::domain::Source`].
    Creation,
    /// Failed to start fetching.
    StartFetching,
    /// Failed to find the CyclicProcess of a Source.
    FindProcess,
    /// Failed to find a Source with the provided ID.
    FindById,
    /// Unable to find the CyclicProcess of a Source.
    ProcessNotFound,
    /// This [Source][`super::domain::Source`] has no possible [Resources][`crate::features::output::domain::resource`].
    NoPossibleResources,
    /// The Source has not been persisted yet.
    NotPersistedYet,
    Unknown,
}

impl DomainErrorKind for SourceErrorKind {
    /// Gets the locale code of this [SourceErrorKind].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.source.creation".to_string(),
            Self::StartFetching => "error.source.start_fetching".to_string(),
            Self::FindProcess => "error.source.find_process".to_string(),
            Self::FindById => "error.source.find_by_id".to_string(),
            Self::ProcessNotFound => "error.source.process_not_found".to_string(),
            Self::NoPossibleResources => "error.source.no_possible_resources".to_string(),
            Self::NotPersistedYet => "error.source.not_persisted_yet".to_string(),
            Self::Unknown => "error.source.unknown".to_string(),
        }
    }

    /// Gets the message of this [SourceErrorKind].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new source.".to_string(),
            Self::StartFetching => "Failed to start fetching.".to_string(),
            Self::FindProcess => "Failed to find the process of a source.".to_string(),
            Self::FindById => "Failed to find a source with the provided ID.".to_string(),
            Self::ProcessNotFound => "Unable to find the process of a source.".to_string(),
            Self::NoPossibleResources => "A source needs possible resources.".to_string(),
            Self::NotPersistedYet => "The source has not been persisted yet.".to_string(),
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

impl Display for SourceErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
