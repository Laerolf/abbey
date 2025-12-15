use std::fmt::Display;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum SourceErrorKind {
    /// Failed to create a new [Source][`super::domain::Source`].
    Creation,
    /// This [Source][`super::domain::Source`] has no possible [Resources][`crate::features::output::domain::resource`].
    NoPossibleResources,
}

impl DomainErrorKind for SourceErrorKind {
    /// Gets the locale code of this [SourceError].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.source.creation".to_string(),
            Self::NoPossibleResources => "error.source.no_possible_resources".to_string(),
        }
    }

    /// Gets the message of this [SourceError].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new source.".to_string(),
            Self::NoPossibleResources => "A source needs possible resources.".to_string(),
        }
    }
}

impl Display for SourceErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
