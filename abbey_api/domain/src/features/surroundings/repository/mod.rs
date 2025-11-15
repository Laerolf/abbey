use entity::surroundings;
use sea_orm::{ActiveModelTrait, DbErr};

use crate::shared::db::DatabasePool;

/// Represents an element that handles all [Surroundings][`super::domain::Surroundings`] database topics.
#[derive(Default, Clone)]
pub struct SurroundingsRepository {}

impl SurroundingsRepository {
    /// Inserts a [Surroundings][`surroundings::Model`].
    pub async fn insert(
        &self,
        new_surroundings: surroundings::ActiveModel,
    ) -> Result<surroundings::Model, DbErr> {
        new_surroundings.insert(DatabasePool::instance()).await
    }
}
