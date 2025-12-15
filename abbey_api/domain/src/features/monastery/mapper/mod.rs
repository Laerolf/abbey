use entity::{monasteries, monastery_monks};
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};

use crate::features::{
    actor::{domain::monk::Monk, mapper::MonkMapper},
    monastery::{
        domain::Monastery, forms::MonasteryCreationForm, repository::MonasteryWithRelations,
    },
};

/// Represents an element that maps [`Monastery`] elements.
pub struct MonasteryMapper;

impl MonasteryMapper {
    /// Maps a [`MonasteryCreationForm`] to a [model][`monasteries::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: MonasteryCreationForm) -> monasteries::ActiveModel {
        monasteries::ActiveModel { id: NotSet }
    }

    /// Maps a [`Monastery`] to a [model][`monasteries::ActiveModel`] to update.
    pub fn to_update_active_model(monastery: Monastery) -> monasteries::ActiveModel {
        monasteries::ActiveModel {
            id: Unchanged(monastery.id),
        }
    }

    /// Maps a [model][`monasteries::Model`] to a [`Monastery`].
    pub fn to_domain_entity(model: monasteries::Model, monks: Vec<Monk>) -> Monastery {
        Monastery::new(model.id, monks)
    }

    /// Maps a [`Monastery`] and a [`Monk`] to a [model][`monastery_monks::ActiveModel`] to update.
    pub fn to_new_monastery_monk_active_model(
        monastery: &Monastery,
        monk: &Monk,
    ) -> monastery_monks::ActiveModel {
        monastery_monks::ActiveModel {
            id: NotSet,
            monastery_id: Set(monastery.id),
            monk_id: Set(monk.id),
        }
    }

    /// Maps a [model][`MonasteryWithRelations`] to a [`Monastery`].
    pub fn to_domain_entity_with_relations(relations: MonasteryWithRelations) -> Monastery {
        let monks = relations
            .monks
            .into_iter()
            .map(MonkMapper::to_domain_entity)
            .collect();

        Monastery::new(relations.monastery.id, monks)
    }
}
