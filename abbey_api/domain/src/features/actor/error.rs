use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum ActorErrorKind {
    /// Failed to create a new [`Actor`][`super::domain::Actor`].
    Creation,
    /// Failed to update an Actor.
    Update,
    /// Failed to find an Actor with the provided ID.
    FindById,
    /// Failed to get an Actor with the provided ID.
    GetById,
    /// Failed to find Actors with the provided IDs.
    FindByIds,
    /// The [`Actor`][`super::domain::Actor`] is not available.
    Assigned,
    /// The Actor has no Skills.
    NoSkills,
    Unknown,
}

impl DomainErrorKind for ActorErrorKind {
    /// Gets the locale code of the [`ActorError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.actor.creation".to_string(),
            Self::Update => "error.actor.update".to_string(),
            Self::FindById => "error.actor.find_by_id".to_string(),
            Self::GetById => "error.actor.find_by_id".to_string(),
            Self::FindByIds => "error.actor.find_by_ids".to_string(),
            Self::Assigned => "error.actor.assigned".to_string(),
            Self::NoSkills => "error.actor.no_skills".to_string(),
            Self::Unknown => "error.actor.unknown".to_string(),
        }
    }

    /// Gets the message of the [`ActorError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new Actor.".to_string(),
            Self::Update => "Failed to update an Actor.".to_string(),
            Self::FindById => "Failed to find an Actor with the provided ID.".to_string(),
            Self::GetById => "Failed to get an Actor with the provided ID.".to_string(),
            Self::FindByIds => "Failed to find Actors with the provided IDs.".to_string(),
            Self::Assigned => "An Actor can only be assigned to one Process.".to_string(),
            Self::NoSkills => "The Actor has no Skills.".to_string(),
            Self::Unknown => "An unknown error occurred.".to_string(),
        }
    }

    fn http_status(&self) -> StatusCode {
        match self {
            Self::FindById => StatusCode::NOT_FOUND,
            Self::GetById => StatusCode::NOT_FOUND,
            Self::FindByIds => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn unknown() -> Self {
        Self::Unknown
    }
}

impl Display for ActorErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
