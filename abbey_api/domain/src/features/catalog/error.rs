use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum CatalogErrorKind {
    /// Failed to get all Skills.
    GetAllSkills,
    /// Failed to get all Resources.
    GetAllResources,
    Unknown,
}

impl DomainErrorKind for CatalogErrorKind {
    /// Gets the locale code of the [`CatalogErrorKind`].
    fn code(&self) -> String {
        match self {
            Self::GetAllSkills => "error.catalog.get_all_skills".to_string(),
            Self::GetAllResources => "error.catalog.get_all_resources".to_string(),
            Self::Unknown => "error.catalog.unknown".to_string(),
        }
    }

    /// Gets the message of the [`CatalogErrorKind`].
    fn message(&self) -> String {
        match self {
            Self::GetAllSkills => "Failed to get all Skills.".to_string(),
            Self::GetAllResources => "Failed to get all Resources.".to_string(),
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

impl Display for CatalogErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
