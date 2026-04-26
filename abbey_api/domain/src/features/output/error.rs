use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum ResourceErrorKind {
    /// Failed to create a new [Resource][`super::domain::resource::Resource`].
    Creation,
    /// Failed to find a Resource by its name.
    FindByName,
    Unknown,
}

impl DomainErrorKind for ResourceErrorKind {
    /// Gets the locale code of a [`ResourceErrorKind`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.resource.creation".to_string(),
            Self::FindByName => "error.resource.find_by_name".to_string(),
            Self::Unknown => "error.resource.unknown".to_string(),
        }
    }

    /// Gets the message of a [`ResourceErrorKind`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new resource.".to_string(),
            Self::FindByName => "Failed to find a Resource by its name.".to_string(),
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
