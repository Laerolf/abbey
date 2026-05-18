use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{features::player::domain::Player, shared::DomainElement};

/// Represents a [Player] DTO.
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema, Clone)]
pub struct PlayerDto {
    /// The ID of the Player.
    #[schema(example = 666)]
    pub id: i32,
    /// The assigned Process ID of the Player.
    pub assigned_process_id: Option<i32>,
}

impl PlayerDto {
    /// Creates a [`PlayerDto`] based on a [Player].
    pub fn from(player: Player) -> Self {
        Self {
            id: player.id().unwrap(),
            assigned_process_id: player
                .assigned_process()
                .as_ref()
                .map(|process| process.id().unwrap()),
        }
    }
}
