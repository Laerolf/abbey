use entity::monks;
use sea_orm::{ActiveModelTrait, DbErr};

use crate::shared::db::DatabasePool;

/// Represents an element that handles all [`Monk`][`super::domain::monk`] database topics.
#[derive(Default, Clone)]
pub struct MonkRepository {}

impl MonkRepository {
    /// Inserts a [`Monk`][`monks::Model`].
    pub async fn insert(&self, new_monk: monks::ActiveModel) -> Result<monks::Model, DbErr> {
        new_monk.insert(DatabasePool::instance()).await
    }
}
