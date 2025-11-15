use entity::players;
use sea_orm::{ActiveModelTrait, DbErr};

use crate::shared::db::DatabasePool;

/// Represents an element that handles all [Player][`crate::features::player::domain::Player`] database topics.
#[derive(Default, Clone)]
pub struct PlayerRepository {}

impl PlayerRepository {
    /// Inserts a [Player][`players::Model`].
    pub async fn insert(&self, new_player: players::ActiveModel) -> Result<players::Model, DbErr> {
        new_player.insert(DatabasePool::instance()).await
    }
}
