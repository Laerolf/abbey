use entity::{games, user_games};
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use time::OffsetDateTime;

use crate::{
    features::{
        game::{
            domain::{Game, game_engine::GameEngine},
            error::GameErrorKind,
            forms::{GameBlueprint, UserGameAssignmentForm},
        },
        monastery::domain::Monastery,
        player::domain::Player,
        surroundings::domain::Surroundings,
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents an element that maps [`Game`] elements.
pub struct GameMapper;

impl GameMapper {
    /// Maps a [`GameBlueprint`] to a [model][`games::ActiveModel`] to create.
    pub fn to_new_active_model(blueprint: GameBlueprint) -> games::ActiveModel {
        games::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            engine_state: Set(blueprint.engine_state),
            monastery_id: Set(blueprint.monastery_id),
            player_id: Set(blueprint.player_id),
            surroundings_id: Set(blueprint.surroundings_id),
        }
    }

    /// Maps a [`Game`] to a [model][`games::ActiveModel`] to update.
    pub fn to_update_active_model(
        game: Game,
    ) -> Result<games::ActiveModel, DomainError<GameErrorKind>> {
        Ok(games::ActiveModel {
            id: Unchanged(game.id().unwrap()),
            created_at: Unchanged(game.created_at().unwrap()),
            last_updated_at: Set(Some(OffsetDateTime::now_utc())),
            engine_state: Set(game.game_engine().as_string()?),
            monastery_id: Unchanged(game.monastery().id().unwrap()),
            player_id: Unchanged(game.player().id().unwrap()),
            surroundings_id: Unchanged(game.surroundings().id().unwrap()),
        })
    }

    /// Maps a [model][games::Model] to a [`Game`].
    pub fn to_domain_entity(
        game: games::Model,
        engine: GameEngine,
        player: Player,
        monastery: Monastery,
        surroundings: Surroundings,
    ) -> Game {
        Game::restore(
            game.id,
            game.created_at,
            game.last_updated_at,
            engine,
            player,
            monastery,
            surroundings,
        )
    }
}

/// Represents a mapper for [`UserGames`][user_games::Entity].
pub struct UserGameMapper;

impl UserGameMapper {
    /// Maps a [UserGameAssignmentForm] to a new [`model`][user_games::ActiveModel].
    pub fn to_new_active_model(form: UserGameAssignmentForm) -> user_games::ActiveModel {
        user_games::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            user_id: Set(form.user_id),
            game_id: Set(form.game_id),
        }
    }
}
