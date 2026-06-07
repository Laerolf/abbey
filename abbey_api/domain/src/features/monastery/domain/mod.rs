use time::OffsetDateTime;

use crate::{
    features::{monastery::error::MonasteryErrorKind, monk::domain::Monk},
    shared::{DomainElement, error::DomainError},
};

#[derive(Clone)]
pub struct Monastery {
    /// The ID of this [`Monastery`].
    id: Option<i32>,

    /// The creation date of this [`Monastery`].
    created_at: Option<OffsetDateTime>,

    /// The date of the last update of this [`Monastery`].
    last_updated_at: Option<OffsetDateTime>,

    /// The [Monks][`crate::features::actor::domain::monk`] in a [`Monastery`].
    monks: Vec<Monk>,
}

impl Monastery {
    /// Creates a new [`Monastery`].
    pub fn new(monks: Vec<Monk>) -> Result<Self, DomainError<MonasteryErrorKind>> {
        if monks.is_empty() {
            return Err(DomainError::from(MonasteryErrorKind::MissingMonks));
        }

        Ok(Self {
            id: None,
            created_at: None,
            last_updated_at: None,
            monks,
        })
    }

    /// Creates a [`Monastery`].
    pub fn restore(
        id: i32,
        created_at: OffsetDateTime,
        last_updated_at: Option<OffsetDateTime>,
        monks: Vec<Monk>,
    ) -> Result<Self, DomainError<MonasteryErrorKind>> {
        if monks.is_empty() {
            return Err(DomainError::from(MonasteryErrorKind::MissingMonks));
        }

        Ok(Self {
            id: Some(id),
            created_at: Some(created_at),
            last_updated_at,
            monks,
        })
    }

    /// Gets the [monks][Vec<Monk>] of this [`Monastery`].
    pub fn monks(&self) -> &Vec<Monk> {
        &self.monks
    }
}

impl DomainElement<MonasteryErrorKind> for Monastery {
    /// Gets the ID of this [`Monastery`].
    fn id(&self) -> Result<i32, DomainError<MonasteryErrorKind>> {
        self.id
            .ok_or(DomainError::from(MonasteryErrorKind::NotPersistedYet))
    }

    /// Gets the [creation date][`OffsetDateTime`] of this [`Monastery`].
    fn created_at(&self) -> &Option<OffsetDateTime> {
        &self.created_at
    }

    /// Gets the [latest update date][`OffsetDateTime`] of this [`Monastery`].
    fn last_updated_at(&self) -> &Option<OffsetDateTime> {
        &self.last_updated_at
    }
}
