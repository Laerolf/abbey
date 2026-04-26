use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::features::{
    player::domain::Player,
    process::{domain::ProcessKind, dto::ProcessDto},
};

/// Represents a [Player] DTO.
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct PlayerDto {
    /// The ID of the Player.
    #[schema(example = 666)]
    pub id: i32,
    /// The Process of the Player.
    pub process: Option<ProcessDto>,
}

impl PlayerDto {
    /// Creates a [`PlayerDto`] based on a [Player].
    pub fn from(player: Player) -> Self {
        Self {
            id: player.id().unwrap(),
            process: player
                .assigned_process()
                .as_ref()
                .map(|process: &ProcessKind| ProcessDto::from(process.clone())),
        }
    }
}
