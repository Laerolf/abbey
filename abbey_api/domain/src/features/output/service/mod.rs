use sea_orm::DatabaseTransaction;

use crate::{
    features::output::{
        domain::resource::{Category, Resource},
        error::ResourceErrorKind,
        forms::ResourceCreationForm,
        repository::ResourceRepository,
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Resource`] topic.
#[derive(Clone)]
pub struct ResourceService {
    repository: ResourceRepository,
}

impl ResourceService {
    /// Creates a new [`ResourceService`].
    pub fn new(repository: ResourceRepository) -> Self {
        Self { repository }
    }

    /// Finds a [`Resource`] by its name, if not found, the resource will be created.
    pub async fn find_by_name_or_create(
        &self,
        name: impl Into<String>,
        category: Category,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Resource, DomainError<ResourceErrorKind>> {
        let creation_form = ResourceCreationForm::new(name, category.to_string());

        self.repository
            .find_by_name_or_create(creation_form, db_transaction)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::Creation).with_cause(error))
    }

    /// Creates a new [`Resource`].
    pub async fn create_resource(
        &self,
        name: impl Into<String>,
        category: Category,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Resource, DomainError<ResourceErrorKind>> {
        let creation_form = ResourceCreationForm::new(name, category.to_string());

        self.repository
            .create(creation_form, db_transaction)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::Creation).with_cause(error))
    }
}
