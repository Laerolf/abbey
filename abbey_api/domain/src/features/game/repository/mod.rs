use entity::games;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait};

use crate::shared::db::DatabasePool;

/// Represents an element that handles all [Game][`crate::features::game::domain::Game`] database topics.
#[derive(Default, Clone)]
pub struct GameRepository;

impl GameRepository {
    /// Inserts a [Game][`games::Model`].
    pub async fn insert(&self, new_game: games::ActiveModel) -> Result<games::Model, DbErr> {
        new_game.insert(DatabasePool::instance()).await
    }

    /// Finds a [Game][`games::Model`] by its ID.
    pub async fn find_by_id(&self, id: i32) -> Result<Option<games::Model>, DbErr> {
        games::Entity::find_by_id(id)
            .one(DatabasePool::instance())
            .await
    }
}
