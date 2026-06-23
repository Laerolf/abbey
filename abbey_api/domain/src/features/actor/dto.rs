use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::features::{actor::domain::ActorKind, monk::dto::MonkDto, player::dto::PlayerDto};

#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema, Clone)]
#[serde(tag = "type")]
pub enum ActorDto {
    /// The [Player][`PlayerDto`] variant.
    Player(PlayerDto),
    /// The [Monk][`MonkDto`] variant.
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

    pub fn id(&self) -> i32 {
        match self {
            ActorDto::Player(player_dto) => player_dto.id,
            ActorDto::Monk(monk_dto) => monk_dto.id,
        }
    }
}
