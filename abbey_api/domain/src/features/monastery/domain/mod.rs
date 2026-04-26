use crate::{
    features::{actor::domain::monk::Monk, monastery::error::MonasteryErrorKind},
    shared::error::DomainError,
};

#[derive(Clone)]
pub struct Monastery {
    /// The ID of this [`Monastery`].
    id: Option<i32>,

    /// The [Monks][`crate::features::actor::domain::monk`] in a [`Monastery`].
    monks: Vec<Monk>,
}

impl Monastery {
    /// Creates a new [`Monastery`].
    pub fn new(monks: Vec<Monk>) -> Result<Self, DomainError<MonasteryErrorKind>> {
        if monks.is_empty() {
            return Err(DomainError::from(MonasteryErrorKind::MissingMonks));
        }

        Ok(Self { id: None, monks })
    }

    /// Creates a [`Monastery`].
    pub fn restore(id: i32, monks: Vec<Monk>) -> Result<Self, DomainError<MonasteryErrorKind>> {
        if monks.is_empty() {
            return Err(DomainError::from(MonasteryErrorKind::MissingMonks));
        }

        Ok(Self {
            id: Some(id),
            monks,
        })
    }

    /// Gets the ID of this [`Monastery`].
    pub fn id(&self) -> &Option<i32> {
        &self.id
    }

    /// Gets the [monks][Vec<Monk>] of this [`Monastery`].
    pub fn monks(&self) -> &Vec<Monk> {
        &self.monks
    }
}
