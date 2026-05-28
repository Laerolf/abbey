use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum MonasteryErrorKind {
    /// Failed to create a new Monastery.
    Creation,
    /// Failed to restore a Monastery.
    Restore,
    /// The Monks of a Monastery are missing.
    MissingMonks,
    /// Failed to find a Monastery with the provided ID.
    FindById,
    /// Failed to get a Monastery with the provided ID.
    GetById,
    /// Failed to get the Monasteries with the provided IDs.
    GetByIds,
    /// Failed to find all Monks for a Monastery.
    GetAllMonks,
    /// The Monastery has not been persisted yet.
    NotPersistedYet,
    Unknown,
}

impl DomainErrorKind for MonasteryErrorKind {
    /// Gets the locale code of a [`MonasteryErrorKind`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.monastery.creation".to_string(),
            Self::Restore => "error.monastery.restore".to_string(),
            Self::MissingMonks => "error.monastery.missing_monks".to_string(),
            Self::FindById => "error.monastery.find_by_id".to_string(),
            Self::GetById => "error.monastery.get_by_id".to_string(),
            Self::GetByIds => "error.monastery.get_by_ids".to_string(),
            Self::GetAllMonks => "error.monastery.get_all_monks".to_string(),
            Self::NotPersistedYet => "error.monastery.not_persisted_yet".to_string(),
            Self::Unknown => "error.monastery.unknown".to_string(),
        }
    }

    /// Gets the message of a [`MonasteryErrorKind`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new Monastery.".to_string(),
            Self::Restore => "Failed to create a Monastery.".to_string(),
            Self::MissingMonks => "The Monks of a Monastery are missing.".to_string(),
            Self::FindById => "Failed to find a Monastery with the provided ID.".to_string(),
            Self::GetById => "Failed to get a Monastery with the provided ID.".to_string(),
            Self::GetByIds => "Failed to get the Monasteries with the provided IDs.".to_string(),
            Self::GetAllMonks => "Failed to find all Monks for a Monastery.".to_string(),
            Self::NotPersistedYet => "The Monastery has not been persisted yet.".to_string(),
            Self::Unknown => "An unknown error occurred.".to_string(),
        }
    }

    fn http_status(&self) -> StatusCode {
        match self {
            Self::FindById => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn unknown() -> Self {
        Self::Unknown
    }
}

impl Display for MonasteryErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
