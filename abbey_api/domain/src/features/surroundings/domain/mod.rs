use time::OffsetDateTime;

use crate::{
    features::{source::domain::Source, surroundings::error::SurroundingsErrorKind},
    shared::{DomainElement, error::DomainError},
};

/// Represents the surroundings of a [Monastery][`crate::features::monastery::domain::Monastery`].
#[derive(Clone)]
pub struct Surroundings {
    /// The ID of this [`Surroundings`].
    id: Option<i32>,

    /// The creation date of this [`Surroundings`].
    created_at: Option<OffsetDateTime>,

    /// The date of the last update of this [`Surroundings`].
    last_updated_at: Option<OffsetDateTime>,

    /// The [Sources][`crate::features::source::domain::Source`] belonging to this [`Surroundings`].
    sources: Vec<Source>,
}

impl Surroundings {
    /// Creates a new [`Surroundings`].
    pub fn new(sources: Vec<Source>) -> Self {
        Self {
            id: None,
            created_at: None,
            last_updated_at: None,
            sources,
        }
    }

    /// Creates [`Surroundings`].
    pub fn restore(
        id: i32,
        created_at: OffsetDateTime,
        last_updated_at: Option<OffsetDateTime>,
        sources: Vec<Source>,
    ) -> Result<Self, DomainError<SurroundingsErrorKind>> {
        if sources.is_empty() {
            return Err(DomainError::from(SurroundingsErrorKind::MissingSources));
        };

        Ok(Self {
            id: Some(id),
            created_at: Some(created_at),
            last_updated_at,
            sources,
        })
    }

    /// Gets the [Sources][`Vec<Source>`] of this [`Surroundings`].
    pub fn sources(&self) -> &Vec<Source> {
        &self.sources
    }
}

impl DomainElement<SurroundingsErrorKind> for Surroundings {
    /// Gets the ID of this [`Surroundings`].
    fn id(&self) -> Result<i32, DomainError<SurroundingsErrorKind>> {
        self.id
            .ok_or(DomainError::from(SurroundingsErrorKind::NotPersistedYet))
    }

    /// Gets the [creation date][`OffsetDateTime`] of this [`Surroundings`].
    fn created_at(&self) -> &Option<OffsetDateTime> {
        &self.created_at
    }

    /// Gets the [latest update date][`OffsetDateTime`] of this [`Surroundings`].
    fn last_updated_at(&self) -> &Option<OffsetDateTime> {
        &self.last_updated_at
    }
}
