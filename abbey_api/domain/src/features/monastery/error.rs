use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum MonasteryErrorKind {
    /// Failed to create a new [Monastery][`crate::features::monastery::domain::Monastery`].
    Creation,
    /// The Monks of a Monastery are missing.
    MissingMonks,
    /// Failed to find a [Monastery][`crate::features::monastery::domain::Monastery`] with the provided ID.
    FindById,
    /// Failed to find all Monks for a Monastery.
    GetAllMonks,
    Unknown,
}

impl DomainErrorKind for MonasteryErrorKind {
    /// Gets the locale code of a [`MonasteryError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.monastery.creation".to_string(),
            Self::MissingMonks => "error.monastery.missing_monks".to_string(),
            Self::FindById => "error.monastery.find_by_id".to_string(),
            Self::GetAllMonks => "error.monastery.get_all_monks".to_string(),
            Self::Unknown => "error.monastery.unknown".to_string(),
        }
    }

    /// Gets the message of a [`MonasteryError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new monastery.".to_string(),
            Self::MissingMonks => "The Monks of a Monastery are missing.".to_string(),
            Self::FindById => "Failed to find a monastery with the provided ID.".to_string(),
            Self::GetAllMonks => "Failed to find all Monks for a Monastery.".to_string(),
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
