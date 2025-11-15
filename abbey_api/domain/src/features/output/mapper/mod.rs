use std::str::FromStr;

use entity::resources;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::features::output::{
    domain::resource::{Category, Resource},
    forms::ResourceCreationForm,
};

/// Represents an element that maps [`Resource`] elements.
pub struct ResourceMapper;

impl ResourceMapper {
    /// Maps a [`ResourceCreationForm`] to a [model][`resources::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: ResourceCreationForm) -> resources::ActiveModel {
        resources::ActiveModel {
            id: NotSet,
            name: Set(creation_form.name),
            category: Set(creation_form.category_name),
        }
    }

    /// Maps a [`Resource`] to a [model][`resources::ActiveModel`] to update.
    pub fn to_update_active_model(resource: &Resource) -> resources::ActiveModel {
        resources::ActiveModel {
            id: Set(resource.id),
            name: Set(resource.name.clone()),
            category: Set(resource.category.to_string()),
        }
    }

    /// Maps a [model][`resources::Model`] to a [`Resource`].
    pub fn to_domain_entity(model: resources::Model) -> Resource {
        Resource::new(
            model.id,
            model.name,
            Category::from_str(&model.category).expect("A resource should have a valid category."),
        )
    }
}
