use std::fmt::Display;

use crate::shared::error::DomainError;

#[derive(Debug)]
pub enum SourceError {
    /// Failed to create a new [Source][`super::domain::Source`].
    Creation,
    /// This [Source][`super::domain::Source`] has no possible [Resources][`crate::features::output::domain::resource`].
    NoPossibleResources,
}

impl std::error::Error for SourceError {}

impl DomainError for SourceError {
    /// Gets the locale code of this [SourceError].
    fn code(&self) -> &'static str {
        match self {
            Self::Creation => "error.source.creation",
            Self::NoPossibleResources => "error.source.no_possible_resources",
        }
    }

    /// Gets the message of this [SourceError].
    fn message(&self) -> &'static str {
        match self {
            Self::Creation => "Failed to create a new source.",
            Self::NoPossibleResources => "A source needs possible resources.",
        }
    }
}

impl Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
