use entity::resources;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::shared::db::DatabasePool;

/// Represents an element that handles all [Resource][`super::domain::resource::Resource`] database topics.
#[derive(Default)]
pub struct ResourceRepository {}

impl ResourceRepository {
    /// Inserts a [Resource][`resources::Model`].
    pub async fn insert(
        &self,
        new_resource: resources::ActiveModel,
    ) -> Result<resources::Model, DbErr> {
        new_resource.insert(DatabasePool::instance()).await
    }

    /// Finds a [Resource][`resources::Model`] by its name.
    pub async fn find_by_name(&self, name: &String) -> Result<Option<resources::Model>, DbErr> {
        resources::Entity::find()
            .filter(resources::Column::Name.eq(name))
            .one(DatabasePool::instance())
            .await
    }
}
