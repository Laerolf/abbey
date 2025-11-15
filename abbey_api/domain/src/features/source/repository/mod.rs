use entity::sources;
use sea_orm::{ActiveModelTrait, DbErr};

use crate::shared::db::DatabasePool;

/// Represents an element that handles all [Source][`super::domain::Source`] database topics.
#[derive(Default, Clone)]
pub struct SourceRepository {}

impl SourceRepository {
    /// Inserts a [Source][`sources::Model`].
    pub async fn insert(&self, new_source: sources::ActiveModel) -> Result<sources::Model, DbErr> {
        new_source.insert(DatabasePool::instance()).await
    }
}
