use sea_orm::DatabaseTransaction;

use crate::{
    features::output::{
        domain::resource::{Category, Resource},
        error::ResourceErrorKind,
        forms::ResourceCreationForm,
        mapper::ResourceMapper,
        repository::ResourceRepository,
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Resource`] topic.
#[derive(Default, Clone)]
pub struct ResourceService {
    repository: ResourceRepository,
}

impl ResourceService {
    /// Finds a [`Resource`] by its name, if not found, the resource will be created.
    pub async fn find_by_name_or_create_in_transaction(
        &self,
        name: impl Into<String>,
        category: Category,
        transaction: &DatabaseTransaction,
    ) -> Result<Resource, DomainError<ResourceErrorKind>> {
        let creation_form = ResourceCreationForm::new(name, category.to_string());

        let resource = self
            .repository
            .find_by_name_or_create_in_transaction(creation_form, transaction)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::Creation).with_cause(error))?;

        Ok(ResourceMapper::to_domain_entity(resource))
    }

    /// Creates a new [`Resource`].
    pub async fn create_resource_in_transaction(
        &self,
        name: impl Into<String>,
        category: Category,
        transaction: &DatabaseTransaction,
    ) -> Result<Resource, DomainError<ResourceErrorKind>> {
        let creation_form = ResourceCreationForm::new(name, category.to_string());

        let resource = self
            .repository
            .create_in_transaction(creation_form, transaction)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::Creation).with_cause(error))?;

        Ok(ResourceMapper::to_domain_entity(resource))
    }
}
