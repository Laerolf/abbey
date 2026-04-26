use entity::{monasteries, monastery_monks};
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};

use crate::{
    features::{
        actor::domain::{Actor, monk::Monk},
        monastery::{domain::Monastery, error::MonasteryErrorKind},
    },
    shared::error::DomainError,
};

/// Represents an element that maps [`Monastery`] elements.
pub struct MonasteryMapper;

impl MonasteryMapper {
    /// Creates a new [model][`monasteries::ActiveModel`].
    pub fn to_new_active_model() -> monasteries::ActiveModel {
        monasteries::ActiveModel { id: NotSet }
    }

    /// Maps a [`Monastery`] to a [model][`monasteries::ActiveModel`] to update.
    pub fn to_update_active_model(monastery: Monastery) -> monasteries::ActiveModel {
        monasteries::ActiveModel {
            id: Unchanged(monastery.id().unwrap()),
        }
    }

    /// Maps a [model][`monasteries::Model`] to a [`Monastery`].
    pub fn to_domain_entity(
        model: monasteries::Model,
        monks: Vec<Monk>,
    ) -> Result<Monastery, DomainError<MonasteryErrorKind>> {
        Monastery::restore(model.id, monks)
    }

    /// Maps a [`Monastery`] and a [`Monk`] to a [model][`monastery_monks::ActiveModel`] to update.
    pub fn to_new_monastery_monk_active_model(
        monastery: &Monastery,
        monk: &Monk,
    ) -> monastery_monks::ActiveModel {
        monastery_monks::ActiveModel {
            id: NotSet,
            monastery_id: Set(monastery.id().unwrap()),
            monk_id: Set(monk.id().unwrap()),
        }
    }
}
