use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::features::{source::dto::SourceDto, surroundings::domain::Surroundings};

/// Represents a [Surroundings] DTO.
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct SurroundingsDto {
    /// The ID of the Surroundings.
    #[schema(example = 666)]
    pub id: i32,
    /// The Sources  of the Surroundings.
    pub sources: Vec<SourceDto>,
}

impl SurroundingsDto {
    /// Creates a [`SurroundingsDto`] based on a [Surroundings].
    pub fn from(surroundings: Surroundings) -> Self {
        Self {
            id: surroundings.id().unwrap(),
            sources: surroundings
                .sources()
                .clone()
                .into_iter()
                .map(SourceDto::from)
                .collect(),
        }
    }
}
