use entity::monastery;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::features::monastery::{domain::Monastery, forms::MonasteryCreationForm};

/// Represents an element that maps [`Monastery`] elements.
#[derive(Default)]
pub struct MonasteryMapper;

impl MonasteryMapper {
    /// Maps a [`MonasteryCreationForm`] to a [model][`monastery::ActiveModel`] to create.
    pub fn to_new_active_model(
        &self,
        creation_form: MonasteryCreationForm,
    ) -> monastery::ActiveModel {
        monastery::ActiveModel { id: NotSet }
    }

    /// Maps a [`Monastery`] to a [model][`monastery::ActiveModel`] to update.
    pub fn to_update_active_model(&self, monastery: &Monastery) -> monastery::ActiveModel {
        monastery::ActiveModel {
            id: Set(monastery.id),
        }
    }

    /// Maps a [model][`monastery::Model`] to a [`Monastery`].
    pub fn to_domain_entity(&self, model: monastery::Model) -> Monastery {
        Monastery::new(model.id, vec![])
    }
}
