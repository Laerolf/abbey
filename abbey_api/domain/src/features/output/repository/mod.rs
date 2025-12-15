use entity::resources;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, QueryFilter,
};

use crate::{
    features::output::{forms::ResourceCreationForm, mapper::ResourceMapper},
    shared::db::DatabaseClient,
};

/// Represents an element that handles all [Resource][`super::domain::resource::Resource`] database topics.
#[derive(Default, Clone)]
pub struct ResourceRepository {}

impl ResourceRepository {
    /// Finds a [Resource][`resources::Model`] by its name.
    pub async fn find_by_name(&self, name: &String) -> Result<Option<resources::Model>, DbErr> {
        resources::Entity::find()
            .filter(resources::Column::Name.eq(name))
            .one(DatabaseClient::get_connection())
            .await
    }

    /// Creates a [Resource][`resources::Model`].
    pub async fn create_in_transaction(
        &self,
        form: ResourceCreationForm,
        transaction: &DatabaseTransaction,
    ) -> Result<resources::Model, DbErr> {
        ResourceMapper::to_new_active_model(form)
            .insert(transaction)
            .await
    }

    /// Finds a [Resource][`resources::Model`] by its name or creates it if it doesn't exist.
    pub async fn find_by_name_or_create_in_transaction(
        &self,
        form: ResourceCreationForm,
        transaction: &DatabaseTransaction,
    ) -> Result<resources::Model, DbErr> {
        match self.find_by_name(&form.name).await? {
            Some(model) => Ok(model),
            None => self.create_in_transaction(form, transaction).await,
        }
    }
}
