use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    features::{
        actor::domain::{Actor, ActorKind, monk::Monk, person::Person},
        player::dto::PlayerDto,
        process::dto::ProcessDto,
    },
    shared::DomainElement,
};

/// Represents a Monk DTO.
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct MonkDto {
    /// The ID of the Monk.
    #[schema(example = 666)]
    pub id: i32,
    /// The name of the Monk.
    #[schema(example = "Maurits")]
    pub name: String,
    /// The IDs  of the Monk skills.
    pub skill_ids: Vec<i32>,
    /// The assigned Process of the Monk.
    pub assigned_process: Option<ProcessDto>,
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
            assigned_process: monk.assigned_process().clone().map(ProcessDto::from),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(tag = "type")]
pub enum ActorDto {
    /// The [Player] [`PlayerDto`] variant.
    Player(PlayerDto),
    /// The [Monk] [`MonkDto`] variant.
    Monk(MonkDto),
}

impl ActorDto {
    /// Creates a [`ActorDto`] from an [Actor][`ActorKind`].
    pub fn from(actor: ActorKind) -> Self {
        match actor {
            ActorKind::Player(player) => ActorDto::Player(PlayerDto::from(player.clone())),
            ActorKind::Monk(monk) => ActorDto::Monk(MonkDto::from(monk)),
        }
    }
}
