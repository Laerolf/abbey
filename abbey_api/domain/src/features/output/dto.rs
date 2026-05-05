use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{features::output::domain::resource::Resource, shared::DomainElement};

/// Represents a Resource.
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct ResourceDto {
    /// The ID of the Resource.
    #[schema(example = 666)]
    pub id: i32,
    /// The name of the Resource.
    #[schema(example = "pomegranate")]
    pub name: String,
    /// The category of the Resource.
    #[schema(example = "fruits")]
    pub category: String,
}

impl ResourceDto {
    /// Creates a [`ResourceDto`] based on a [Resource].
    pub fn from(resource: Resource) -> Self {
        Self {
            id: resource.id().unwrap(),
            name: resource.name().to_string(),
            category: resource.category().to_string(),
        }
    }
}
