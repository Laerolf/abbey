use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum ResourceErrorKind {
    /// Failed to create a new [Resource][`super::domain::resource::Resource`].
    Creation,
    /// Failed to get all Resources.
    GetAll,
    /// Failed to get Resources with the provided IDs.
    GetByIds,
    /// Failed to find a Resource by its name.
    FindByName,
    /// Failed to get resources with the provided names.
    GetByNames,
    /// The Resource has not been persisted yet.
    NotPersistedYet,
    Unknown,
}

impl DomainErrorKind for ResourceErrorKind {
    /// Gets the locale code of a [`ResourceErrorKind`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.resource.creation".to_string(),
            Self::GetAll => "error.resource.get_all".to_string(),
            Self::GetByIds => "error.resource.get_by_ids".to_string(),
            Self::FindByName => "error.resource.find_by_name".to_string(),
            Self::GetByNames => "error.resource.get_by_names".to_string(),
            Self::NotPersistedYet => "error.resource.not_persisted_yet".to_string(),
            Self::Unknown => "error.resource.unknown".to_string(),
        }
    }

    /// Gets the message of a [`ResourceErrorKind`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new Resource.".to_string(),
            Self::GetAll => "Failed to get all Resources.".to_string(),
            Self::GetByIds => "Failed to get Resources with the provided IDs.".to_string(),
            Self::FindByName => "Failed to find a Resource by its name.".to_string(),
            Self::GetByNames => "Failed to get resources with the provided names.".to_string(),
            Self::NotPersistedYet => "The Resource has not been persisted yet.".to_string(),
            Self::Unknown => "An unknown error occurred.".to_string(),
        }
    }

    fn http_status(&self) -> StatusCode {
        match self {
            Self::FindByName => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn unknown() -> Self {
        Self::Unknown
    }
}

impl Display for ResourceErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
