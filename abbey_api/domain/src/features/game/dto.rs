use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    features::{
        game::domain::Game, monastery::dto::MonasteryDto, player::dto::PlayerDto,
        surroundings::dto::SurroundingsDto,
    },
    shared::DomainElement,
};

/// Represents the payload used to create a new game.
#[derive(Clone, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateGameRequest {
    /// The game seed of the game to create.
    #[schema(example = "666666")]
    pub game_seed: Option<u64>,
}

/// Represents options for a Game to create.
pub struct GameOptionsForm {
    pub game_seed: Option<u64>,
}

impl GameOptionsForm {
    /// Creates an empty [`GameOptionsForm`].
    pub fn empty() -> Self {
        Self { game_seed: None }
    }

    /// Creates a [`GameOptionsForm`] from a [CreateGameRequest].
    pub fn from(request: CreateGameRequest) -> Self {
        Self {
            game_seed: request.game_seed,
        }
    }
}

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
