use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    features::{
        game::domain::Game, monastery::dto::MonasteryDto, player::dto::PlayerDto,
        surroundings::dto::SurroundingsDto,
    },
    shared::DomainElement,
};

/// Represents a [Game] DTO.
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
#[schema(description = "A Game DTO.")]
pub struct GameDto {
    /// The ID of the Game.
    #[schema(example = 666)]
    pub id: i32,
    /// The Player DTO.
    pub player: PlayerDto,
    /// The Monastery DTO.
    pub monastery: MonasteryDto,
    /// The Surroundings DTO.
    pub surroundings: SurroundingsDto,
}

impl GameDto {
    /// Creates a [`GameDto`] based on a [Game].
    pub fn from(game: Game) -> Self {
        Self {
            id: game.id().unwrap(),
            player: PlayerDto::from(game.player().clone()),
            monastery: MonasteryDto::from(game.monastery().clone()),
            surroundings: SurroundingsDto::from(game.surroundings().clone()),
        }
    }
}
