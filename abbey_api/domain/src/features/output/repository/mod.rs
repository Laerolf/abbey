use entity::resources;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use tracing::warn;

use crate::{
    features::output::{
        domain::resource::Resource, error::ResourceErrorKind, forms::ResourceBlueprint,
        mapper::ResourceMapper,
    },
    shared::error::DomainError,
};

/// Represents an element that handles all [Resource][`super::domain::resource::Resource`] database topics.
#[derive(Default, Clone)]
pub struct ResourceRepository;

impl ResourceRepository {
    /// Gets all [`Resources`][Vec<resources::Model>].
    pub async fn get_all<C: ConnectionTrait>(
        &self,
        db_connection: &C,
    ) -> Result<Vec<resources::Model>, DomainError<ResourceErrorKind>> {
        resources::Entity::find()
            .distinct()
            .order_by_asc(resources::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::GetAll).with_cause(error))
    }

    /// Gets the [`Resources`][Vec<resources::Model>] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<resources::Model>, DomainError<ResourceErrorKind>> {
        resources::Entity::find()
            .filter(resources::Column::Id.is_in(ids.to_vec()))
            .distinct()
            .order_by_asc(resources::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::GetByIds).with_cause(error))
    }

    /// Finds a [Resource] by its name.
    pub async fn find_by_name<C: ConnectionTrait>(
        &self,
        name: &String,
        db_connection: &C,
    ) -> Result<Option<Resource>, DomainError<ResourceErrorKind>> {
        let resource = resources::Entity::find()
            .filter(resources::Column::Name.eq(name))
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::FindByName).with_cause(error))?
            .map(ResourceMapper::to_domain_entity);

        Ok(resource)
    }

    /// Gets the [`Resources`][Vec<resources::Model>] with the provided names.
    pub async fn get_by_names<C: ConnectionTrait>(
        &self,
        names: &[String],
        db_connection: &C,
    ) -> Result<Vec<resources::Model>, DomainError<ResourceErrorKind>> {
        resources::Entity::find()
            .filter(resources::Column::Name.is_in(names.to_vec()))
            .distinct()
            .order_by_asc(resources::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::GetByNames).with_cause(error))
    }

    /// Creates a [Resource].
    pub async fn create(
        &self,
        form: ResourceBlueprint,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Resource, DomainError<ResourceErrorKind>> {
        let new_resource: resources::Model = ResourceMapper::to_new_active_model(form)
            .insert(db_transaction)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::Creation).with_cause(error))?;

        Ok(ResourceMapper::to_domain_entity(new_resource))
    }

    /// Creates new [`Resources`][Vec<resources::Model>].
    pub async fn create_many<C: ConnectionTrait>(
        &self,
        models: Vec<resources::ActiveModel>,
        db_connection: &C,
    ) -> Result<Vec<resources::Model>, DomainError<ResourceErrorKind>> {
        if models.is_empty() {
            warn!("Skipping this insertion because no models were provided.");
            return Ok(vec![]);
        }

        resources::Entity::insert_many(models)
            .exec_with_returning_many(db_connection)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::Creation).with_cause(error))
    }

    /// Finds a [Resource] by its name or creates it if it doesn't exist.
    pub async fn find_by_name_or_create(
        &self,
        form: ResourceBlueprint,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Resource, DomainError<ResourceErrorKind>> {
        match self.find_by_name(&form.name, db_transaction).await? {
            Some(resource) => Ok(resource),
            None => self.create(form, db_transaction).await,
        }
    }
}
