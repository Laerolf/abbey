use entity::resources;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter,
};

use crate::{
    features::output::{
        domain::resource::Resource, error::ResourceErrorKind, forms::ResourceCreationForm,
        mapper::ResourceMapper,
    },
    shared::error::DomainError,
};

/// Represents an element that handles all [Resource][`super::domain::resource::Resource`] database topics.
#[derive(Default, Clone)]
pub struct ResourceRepository;

impl ResourceRepository {
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

    /// Creates a [Resource].
    pub async fn create(
        &self,
        form: ResourceCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Resource, DomainError<ResourceErrorKind>> {
        let new_resource: resources::Model = ResourceMapper::to_new_active_model(form)
            .insert(db_transaction)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::Creation).with_cause(error))?;

        Ok(ResourceMapper::to_domain_entity(new_resource))
    }

    /// Finds a [Resource] by its name or creates it if it doesn't exist.
    pub async fn find_by_name_or_create(
        &self,
        form: ResourceCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Resource, DomainError<ResourceErrorKind>> {
        match self.find_by_name(&form.name, db_transaction).await? {
            Some(model) => Ok(model),
            None => self.create(form, db_transaction).await,
        }
    }
}
