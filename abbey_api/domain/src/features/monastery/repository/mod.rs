use entity::monastery;
use sea_orm::{ActiveModelTrait, DbErr};

use crate::shared::db::DatabasePool;

/// Represents an element that handles all [Monastery][`crate::features::monastery::domain::Monastery`] database topics.
#[derive(Default)]
pub struct MonasteryRepository {}

impl MonasteryRepository {
    /// Inserts a [Monastery][`monastery::Model`].
    pub async fn insert(
        &self,
        new_monastery: monastery::ActiveModel,
    ) -> Result<monastery::Model, DbErr> {
        new_monastery.insert(DatabasePool::instance()).await
    }
}
