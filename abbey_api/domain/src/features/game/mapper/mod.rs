use entity::games;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};

use crate::features::{
    game::{
        domain::Game, dto::GameResponse, forms::GameCreationForm, repository::GameWithRelations,
    },
    monastery::{mapper::MonasteryMapper, repository::MonasteryWithRelations},
    player::{mapper::PlayerMapper, repository::PlayerWithRelations},
    surroundings::{mapper::SurroundingsMapper, repository::SurroundingsWithRelations},
};

/// Represents an element that maps [`Game`] elements.
pub struct GameMapper;

impl GameMapper {
    /// Maps a [`GameCreationForm`] to a [model][`games::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: GameCreationForm) -> games::ActiveModel {
        games::ActiveModel {
            id: NotSet,
            monastery_id: Set(creation_form.monastery_id),
            player_id: Set(creation_form.player_id),
            surroundings_id: Set(creation_form.surroundings_id),
        }
    }

    /// Maps a [`Game`] to a [model][`games::ActiveModel`] to update.
    pub fn to_update_active_model(game: Game) -> games::ActiveModel {
        games::ActiveModel {
            id: Unchanged(game.id),
            monastery_id: Unchanged(game.monastery.id),
            player_id: Unchanged(game.player.id),
            surroundings_id: Unchanged(game.surroundings.id),
        }
    }

    /// Maps a [model][`GameWithRelations`] to a [`Game`].
    pub fn to_domain_entity_with_relations(
        relations: GameWithRelations,
        related_player: PlayerWithRelations,
        related_monastery: MonasteryWithRelations,
        related_surroundings: SurroundingsWithRelations,
    ) -> Game {
        Game::new(
            relations.game.id,
            PlayerMapper::to_domain_entity_with_relations(related_player),
            MonasteryMapper::to_domain_entity_with_relations(related_monastery),
            SurroundingsMapper::to_domain_entity_with_relations(related_surroundings),
        )
    }

    /// Maps [`Game`] to a [GameResponse].
    pub fn to_response(element: Game) -> GameResponse {
        GameResponse {}
    }
}
