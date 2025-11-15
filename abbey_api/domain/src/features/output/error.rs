use std::fmt::Display;

use crate::shared::error::DomainError;

#[derive(Debug)]
pub enum ResourceError {
    /// Failed to create a new [Resource][`super::domain::resource::Resource`].
    Creation,
}

impl std::error::Error for ResourceError {}

impl DomainError for ResourceError {
    /// Gets the locale code of a [`ResourceError`].
    fn code(&self) -> &'static str {
        match self {
            Self::Creation => "error.resource.creation",
        }
    }

    /// Gets the message of a [`ResourceError`].
    fn message(&self) -> &'static str {
        match self {
            Self::Creation => "Failed to create a new resource.",
        }
    }
}

impl Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
