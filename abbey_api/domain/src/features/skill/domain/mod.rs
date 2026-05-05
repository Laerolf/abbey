use time::OffsetDateTime;

use crate::{
    features::skill::error::SkillErrorKind,
    shared::{DomainElement, error::DomainError},
};

/// Represents a skill.
#[derive(Clone, Debug)]
pub struct Skill {
    /// The ID of this [`Skill`].
    id: Option<i32>,

    /// The creation date of this [`Skill`].
    created_at: Option<OffsetDateTime>,

    /// The date of the last update of this [`Skill`].
    last_updated_at: Option<OffsetDateTime>,

    /// The name of this [`Skill`].
    name: String,
}

impl Skill {
    /// Creates a new [`Skill`].
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: None,
            created_at: None,
            last_updated_at: None,
            name: name.into(),
        }
    }

    /// Creates a [`Skill`].
    pub fn restore(
        id: i32,
        created_at: OffsetDateTime,
        last_updated_at: Option<OffsetDateTime>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id: Some(id),
            created_at: Some(created_at),
            last_updated_at,
            name: name.into(),
        }
    }

    /// Gets the name of this [`Skill`].
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl DomainElement<SkillErrorKind> for Skill {
    /// Gets the ID of this [`Skill`].
    fn id(&self) -> Result<i32, DomainError<SkillErrorKind>> {
        self.id
            .ok_or(DomainError::from(SkillErrorKind::NotPersistedYet))
    }

    /// Gets the [creation date][`OffsetDateTime`] of this [`Skill`].
    fn created_at(&self) -> &Option<OffsetDateTime> {
        &self.created_at
    }

    /// Gets the [latest update date][`OffsetDateTime`] of this [`Skill`].
    fn last_updated_at(&self) -> &Option<OffsetDateTime> {
        &self.last_updated_at
    }
}
