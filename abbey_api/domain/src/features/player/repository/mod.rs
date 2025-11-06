use entity::player;
use sea_orm::{ActiveModelTrait, DbErr};

use crate::shared::db::DatabasePool;

/// Represents an element that handles all [Player][`crate::features::player::domain::Player`] database topics.
#[derive(Default)]
pub struct PlayerRepository {}

impl PlayerRepository {
    /// Inserts a [Player][`player::Model`].
    pub async fn insert(&self, new_player: player::ActiveModel) -> Result<player::Model, DbErr> {
        new_player.insert(DatabasePool::instance()).await
    }
}
