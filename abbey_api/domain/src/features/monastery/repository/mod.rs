use entity::{monasteries, monastery_monks};
use sea_orm::{ActiveModelTrait, DbErr};

use crate::shared::db::DatabasePool;

/// Represents an element that handles all [Monastery][`super::domain::Monastery`] database topics.
#[derive(Default)]
pub struct MonasteryRepository {}

impl MonasteryRepository {
    /// Inserts a [`Monastery`][`monasteries::Model`].
    pub async fn insert(
        &self,
        new_monastery: monasteries::ActiveModel,
    ) -> Result<monasteries::Model, DbErr> {
        new_monastery.insert(DatabasePool::instance()).await
    }
}

/// Represents an element that handles all [`Monastery Monk`][`monastery_monks`] database topics.
#[derive(Default)]
pub struct MonasteryMonksRepository {}

impl MonasteryMonksRepository {
    /// Inserts a [`Monastery Monk`][`monastery_monks::Model`].
    pub async fn insert(
        &self,
        new_monastery_monk: monastery_monks::ActiveModel,
    ) -> Result<monastery_monks::Model, DbErr> {
        new_monastery_monk.insert(DatabasePool::instance()).await
    }
}
