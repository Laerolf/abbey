use entity::{monasteries, monastery_monks};
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use time::OffsetDateTime;

use crate::{
    features::{
        monastery::{
            domain::Monastery,
            error::MonasteryErrorKind,
            forms::{MonasteryBlueprint, MonasteryMonkAssignmentForm},
        },
        monk::domain::Monk,
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents an element that maps [`Monastery`] elements.
pub struct MonasteryMapper;

impl MonasteryMapper {
    /// Creates a new [model][`monasteries::ActiveModel`].
    pub fn to_new_active_model(_blueprint: MonasteryBlueprint) -> monasteries::ActiveModel {
        monasteries::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
        }
    }

    /// Maps a [`Monastery`] to a [model][`monasteries::ActiveModel`] to update.
    pub fn to_update_active_model(monastery: Monastery) -> monasteries::ActiveModel {
        monasteries::ActiveModel {
            id: Unchanged(monastery.id().unwrap()),
            created_at: Unchanged(monastery.created_at().unwrap()),
            last_updated_at: Set(Some(OffsetDateTime::now_utc())),
        }
    }

    /// Maps a [model][`monasteries::Model`] to a [`Monastery`].
    pub fn to_domain_entity(
        model: monasteries::Model,
        monks: Vec<Monk>,
    ) -> Result<Monastery, DomainError<MonasteryErrorKind>> {
        Monastery::restore(model.id, model.created_at, model.last_updated_at, monks)
    }

    /// Maps a [`Monastery`] and a [`Monk`] to a [model][`monastery_monks::ActiveModel`] to update.
    pub fn to_new_monastery_monk_active_model(
        monastery: &Monastery,
        monk: &Monk,
    ) -> monastery_monks::ActiveModel {
        monastery_monks::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            monastery_id: Set(monastery.id().unwrap()),
            monk_id: Set(monk.id().unwrap()),
        }
    }
}

/// Represents a mapper for [`Monastery Monks`][monastery_monks::Entity].
pub struct MonasteryMonkMapper;

impl MonasteryMonkMapper {
    /// Maps a [MonasteryMonkAssignmentForm] to a [`model`][monastery_monks::ActiveModel].
    pub fn to_new_active_model(form: MonasteryMonkAssignmentForm) -> monastery_monks::ActiveModel {
        monastery_monks::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            monastery_id: Set(form.monastery_id),
            monk_id: Set(form.monk_id),
        }
    }
}
