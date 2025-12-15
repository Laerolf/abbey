use entity::surroundings;
use sea_orm::ActiveValue::{NotSet, Unchanged};

use crate::features::surroundings::{
    domain::Surroundings, forms::SurroundingsCreationForm, repository::SurroundingsWithRelations,
};

/// Represents an element that maps [`Surroundings`] elements.
pub struct SurroundingsMapper;

impl SurroundingsMapper {
    /// Maps a [`SurroundingsCreationForm`] to a [model][`surroundings::ActiveModel`] to create.
    pub fn to_new_active_model(
        creation_form: SurroundingsCreationForm,
    ) -> surroundings::ActiveModel {
        surroundings::ActiveModel { id: NotSet }
    }

    /// Maps a [`Surroundings`] to a [model][`surroundings::ActiveModel`] to update.
    pub fn to_update_active_model(surroundings: Surroundings) -> surroundings::ActiveModel {
        surroundings::ActiveModel {
            id: Unchanged(surroundings.id),
        }
    }

    /// Maps a [model][`surroundings::Model`] to a [`Surroundings`].
    pub fn to_domain_entity(model: surroundings::Model) -> Surroundings {
        Surroundings::new(model.id, Vec::new())
    }

    /// Maps a [model][`surroundings::Model`] to a [`Surroundings`].
    pub fn to_domain_entity_with_relations(relations: SurroundingsWithRelations) -> Surroundings {
        Surroundings::new(relations.surroundings.id, relations.sources)
    }
}
