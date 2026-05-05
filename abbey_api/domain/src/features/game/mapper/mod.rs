use entity::{games, user_games};
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use time::OffsetDateTime;

use crate::{
    features::{
        game::{
            domain::Game,
            forms::{GameCreationForm, UserGameCreationForm},
        },
        monastery::domain::Monastery,
        player::domain::Player,
        surroundings::domain::Surroundings,
    },
    shared::DomainElement,
};

/// Represents an element that maps [`Game`] elements.
pub struct GameMapper;

impl GameMapper {
    /// Maps a [`GameCreationForm`] to a [model][`games::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: GameCreationForm) -> games::ActiveModel {
        games::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            monastery_id: Set(creation_form.monastery_id),
            player_id: Set(creation_form.player_id),
            surroundings_id: Set(creation_form.surroundings_id),
        }
    }

    /// Maps a [`Game`] to a [model][`games::ActiveModel`] to update.
    pub fn to_update_active_model(game: Game) -> games::ActiveModel {
        games::ActiveModel {
            id: Unchanged(game.id().unwrap()),
            created_at: Unchanged(game.created_at().unwrap()),
            last_updated_at: Set(Some(OffsetDateTime::now_utc())),
            monastery_id: Unchanged(game.monastery().id().unwrap()),
            player_id: Unchanged(game.player().id().unwrap()),
            surroundings_id: Unchanged(game.surroundings().id().unwrap()),
        }
    }

    /// Maps a [model][games::Model] to a [`Game`].
    pub fn to_domain_entity(
        game: games::Model,
        player: Player,
        monastery: Monastery,
        surroundings: Surroundings,
    ) -> Game {
        Game::restore(
            game.id,
            game.created_at,
            game.last_updated_at,
            player,
            monastery,
            surroundings,
        )
    }
}

/// Represents a mapper for [`UserGames`][user_games::Entity].
pub struct UserGameMapper;

impl UserGameMapper {
    /// Maps a [UserGameCreationForm] to a new [`model`][user_games::ActiveModel].
    pub fn to_new_active_model(creation_form: UserGameCreationForm) -> user_games::ActiveModel {
        user_games::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            user_id: Set(creation_form.user_id),
            game_id: Set(creation_form.game_id),
        }
    }
}
