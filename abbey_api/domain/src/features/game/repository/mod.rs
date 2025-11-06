use entity::game;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait};

use crate::shared::db::DatabasePool;

/// Represents an element that handles all [Game][`crate::features::game::domain::Game`] database topics.
#[derive(Default)]
pub struct GameRepository;

impl GameRepository {
    /// Inserts a [Game][`game::Model`].
    pub async fn insert(&self, new_game: game::ActiveModel) -> Result<game::Model, DbErr> {
        new_game.insert(DatabasePool::instance()).await
    }

    /// Finds a [Game][`game::Model`] by its ID.
    pub async fn find_by_id(&self, id: i32) -> Result<Option<game::Model>, DbErr> {
        game::Entity::find_by_id(id)
            .one(DatabasePool::instance())
            .await
    }
}
