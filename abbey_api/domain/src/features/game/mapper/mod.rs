use entity::games;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::features::{
    game::{domain::Game, forms::GameCreationForm},
    monastery::domain::Monastery,
    player::domain::Player,
    surroundings::domain::Surroundings,
};

/// Represents an element that maps [`Game`] elements.
pub struct GameMapper;

impl GameMapper {
    /// Maps a [`GameCreationForm`] to a [model][`game::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: GameCreationForm) -> games::ActiveModel {
        games::ActiveModel {
            id: NotSet,
            monastery_id: Set(creation_form.monastery_id),
            player_id: Set(creation_form.player_id),
            surroundings_id: Set(creation_form.surroundings_id),
        }
    }

    /// Maps a [`Game`] to a [model][`game::ActiveModel`] to update.
    pub fn to_update_active_model(game: &Game) -> games::ActiveModel {
        games::ActiveModel {
            id: Set(game.id),
            monastery_id: Set(game.monastery.id),
            player_id: Set(game.player.id),
            surroundings_id: Set(game.surroundings.id),
        }
    }

    /// Maps a [model][`game::Model`], a [Player], [Monastery] and [Surroundings] to a [`Game`].
    pub fn to_domain_entity(
        model: games::Model,
        player: Player,
        monastery: Monastery,
        surroundings: Surroundings,
    ) -> Game {
        Game::new(model.id, player, monastery, surroundings)
    }
}
