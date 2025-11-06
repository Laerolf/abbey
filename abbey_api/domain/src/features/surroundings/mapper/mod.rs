use entity::surroundings;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::features::{
    source::domain::Source,
    surroundings::{domain::Surroundings, forms::SurroundingsCreationForm},
};

/// Represents an element that maps [`Surroundings`] elements.
#[derive(Default)]
pub struct SurroundingsMapper;

impl SurroundingsMapper {
    /// Maps a [`SurroundingsCreationForm`] to a [model][`surroundings::ActiveModel`] to create.
    pub fn to_new_active_model(
        &self,
        creation_form: SurroundingsCreationForm,
    ) -> surroundings::ActiveModel {
        surroundings::ActiveModel { id: NotSet }
    }

    /// Maps a [`Surroundings`] to a [model][`surroundings::ActiveModel`] to update.
    pub fn to_update_active_model(&self, surroundings: &Surroundings) -> surroundings::ActiveModel {
        surroundings::ActiveModel {
            id: Set(surroundings.id),
        }
    }

    /// Maps a [model][`surroundings::Model`] to a [`Surroundings`].
    pub fn to_domain_entity(
        &self,
        model: surroundings::Model,
        sources: Vec<Source>,
    ) -> Surroundings {
        Surroundings::new(model.id, sources)
    }
}
