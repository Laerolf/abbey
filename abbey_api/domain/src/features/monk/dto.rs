use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    features::{
        actor::domain::{Actor, person::Person},
        monk::domain::Monk,
    },
    shared::DomainElement,
};

/// Represents a Monk DTO.
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema, Clone)]
pub struct MonkDto {
    /// The ID of the Monk.
    #[schema(example = 666)]
    pub id: i32,
    /// The name of the Monk.
    #[schema(example = "Maurits")]
    pub name: String,
    /// The IDs  of the Monk skills.
    pub skill_ids: Vec<i32>,
    /// The assigned Process ID of the Monk.
    pub assigned_process_id: Option<i32>,
}

impl MonkDto {
    /// Creates a [`MonkDto`] based on a [Monk].
    pub fn from(monk: Monk) -> Self {
        let skill_ids: Vec<i32> = monk
            .skills()
            .iter()
            .map(|skill| skill.id().unwrap())
            .collect();

        Self {
            id: monk.id().unwrap(),
            name: monk.name().to_string(),
            skill_ids,
            assigned_process_id: monk
                .assigned_process()
                .as_ref()
                .map(|process| process.id().unwrap()),
        }
    }
}
