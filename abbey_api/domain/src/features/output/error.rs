use std::fmt::Display;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum ResourceErrorKind {
    /// Failed to create a new [Resource][`super::domain::resource::Resource`].
    Creation,
}

impl DomainErrorKind for ResourceErrorKind {
    /// Gets the locale code of a [`ResourceError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.resource.creation".to_string(),
        }
    }

    /// Gets the message of a [`ResourceError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new resource.".to_string(),
        }
    }
}

impl Display for ResourceErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
