use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

use crate::{
    features::{process::dto::CyclicProcessDto, source::domain::Source},
    shared::DomainElement,
};

/// Represents a [Source] DTO.
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct SourceDto {
    /// The ID of the Source.
    #[schema(example = 666)]
    pub id: i32,
    /// The name of the Source.
    #[schema(example = "The Sea")]
    pub name: String,
    /// The process of the Source.
    pub process: CyclicProcessDto,
    /// The last time a claim was made for the output of the completed cycles of the Source.
    pub last_claim_at: Option<OffsetDateTime>,
}

impl SourceDto {
    /// Creates a [`SourceDto`] based on a [Source].
    pub fn from(source: Source) -> Self {
        Self {
            id: source.id().unwrap(),
            name: source.name().to_string(),
            process: CyclicProcessDto::from(source.process().clone()),
            last_claim_at: *source.last_claim_at(),
        }
    }
}
