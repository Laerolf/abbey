use std::str::FromStr;

use entity::resources;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use time::OffsetDateTime;

use crate::{
    features::output::{
        domain::resource::{Category, Resource},
        forms::ResourceCreationForm,
    },
    shared::DomainElement,
};

/// Represents an element that maps [`Resource`] elements.
pub struct ResourceMapper;

impl ResourceMapper {
    /// Maps a [`ResourceCreationForm`] to a [model][`resources::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: ResourceCreationForm) -> resources::ActiveModel {
        resources::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            name: Set(creation_form.name),
            category: Set(creation_form.category_name),
        }
    }

    /// Maps a [`Resource`] to a [model][`resources::ActiveModel`] to update.
    pub fn to_update_active_model(resource: Resource) -> resources::ActiveModel {
        resources::ActiveModel {
            id: Unchanged(resource.id().unwrap()),
            created_at: Unchanged(resource.created_at().unwrap()),
            last_updated_at: Set(Some(OffsetDateTime::now_utc())),
            name: Unchanged(resource.name().to_string()),
            category: Unchanged(resource.category().to_string()),
        }
    }

    /// Maps a [model][`resources::Model`] to a [`Resource`].
    pub fn to_domain_entity(model: resources::Model) -> Resource {
        Resource::restore(
            model.id,
            model.created_at,
            model.last_updated_at,
            model.name,
            Category::from_str(&model.category).expect("A resource should have a valid category."),
        )
    }
}
