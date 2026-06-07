use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    features::{monastery::domain::Monastery, monk::dto::MonkDto},
    shared::DomainElement,
};

/// Represents a [Monastery] DTO.
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct MonasteryDto {
    /// The ID of the Monastery.
    #[schema(example = 666)]
    pub id: i32,
    /// The Monks of the Monastery.
    pub monks: Vec<MonkDto>,
}

impl MonasteryDto {
    /// Creates a [`MonasteryDto`] based on a [Monastery].
    pub fn from(monastery: Monastery) -> Self {
        Self {
            id: monastery.id().unwrap(),
            monks: monastery
                .monks()
                .clone()
                .into_iter()
                .map(MonkDto::from)
                .collect(),
        }
    }
}
