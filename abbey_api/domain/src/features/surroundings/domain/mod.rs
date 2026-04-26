use crate::{
    features::{source::domain::Source, surroundings::error::SurroundingsErrorKind},
    shared::error::DomainError,
};

/// Represents the surroundings of a [Monastery][`crate::features::monastery::domain::Monastery`].
#[derive(Clone)]
pub struct Surroundings {
    /// The ID of this [`Surroundings`].
    id: Option<i32>,

    /// The [Sources][`crate::features::source::domain::Source`] belonging to this [`Surroundings`].
    sources: Vec<Source>,
}

impl Surroundings {
    /// Creates a new [`Surroundings`].
    pub fn new(sources: Vec<Source>) -> Self {
        Self { id: None, sources }
    }

    /// Creates [`Surroundings`].
    pub fn restore(
        id: i32,
        sources: Vec<Source>,
    ) -> Result<Self, DomainError<SurroundingsErrorKind>> {
        if sources.is_empty() {
            return Err(DomainError::from(SurroundingsErrorKind::MissingSources));
        };

        Ok(Self {
            id: Some(id),
            sources,
        })
    }

    /// Gets the ID of this [`Surroundings`].
    pub fn id(&self) -> &Option<i32> {
        &self.id
    }

    /// Gets the [Sources][`Vec<Source>`] of this [`Surroundings`].
    pub fn sources(&self) -> &Vec<Source> {
        &self.sources
    }
}
