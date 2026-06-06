use sea_orm::ConnectionTrait;
use tracing::info;

use crate::{
    features::output::{
        domain::resource::Resource, error::ResourceErrorKind, forms::ResourceBlueprint,
        mapper::ResourceMapper, repository::ResourceRepository,
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a command service for [`Resources`][Resource].
#[derive(Clone)]
pub struct ResourceCommandService {
    repository: ResourceRepository,
    resource_query_service: ResourceQueryService,
}

impl ResourceCommandService {
    /// Creates a new [`ResourceCommandService`].
    pub fn new(
        repository: ResourceRepository,
        resource_query_service: ResourceQueryService,
    ) -> Self {
        Self {
            repository,
            resource_query_service,
        }
    }

    /// Creates new [`Resources`][Vec<Resource>].
    async fn create_resources<C: ConnectionTrait>(
        &self,
        blueprints: Vec<ResourceBlueprint>,
        db_connection: &C,
    ) -> Result<Vec<Resource>, DomainError<ResourceErrorKind>> {
        let active_models = blueprints
            .into_iter()
            .map(ResourceMapper::to_new_active_model)
            .collect();

        let models = self
            .repository
            .create_many(active_models, db_connection)
            .await?;

        info!("Created {:?} new Resources", models.len());

        Ok(models
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect())
    }

    /// Gets [`Resources`][Vec<Resource>] with the provided names, if not found, the resources will be created.
    pub async fn get_or_create_many<C: ConnectionTrait>(
        &self,
        blueprints: Vec<ResourceBlueprint>,
        db_connection: &C,
    ) -> Result<Vec<Resource>, DomainError<ResourceErrorKind>> {
        let names: Vec<String> = blueprints
            .clone()
            .into_iter()
            .map(|blueprint| blueprint.name)
            .collect();

        let existing_models = self
            .resource_query_service
            .get_by_names(&names, db_connection)
            .await?;

        let existing_names: Vec<String> = existing_models
            .iter()
            .map(|model| model.name().to_string())
            .collect();

        let missing_model_blueprints: Vec<ResourceBlueprint> = blueprints
            .into_iter()
            .filter(|blueprint| !existing_names.contains(&blueprint.name))
            .collect();

        let new_models = self
            .create_resources(missing_model_blueprints, db_connection)
            .await?;

        let mut all_models: Vec<Resource> = existing_models.into_iter().chain(new_models).collect();
        all_models.sort_by_key(|resource| resource.id().ok());

        Ok(all_models)
    }
}

/// Represents a query service for [`Resources`][Resource].
#[derive(Clone)]
pub struct ResourceQueryService {
    repository: ResourceRepository,
}

impl ResourceQueryService {
    /// Creates a new [`ResourceQueryService`].
    pub fn new(repository: ResourceRepository) -> Self {
        Self { repository }
    }

    /// Gets the [`Resources`][Vec<Resource>] with the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<Resource>, DomainError<ResourceErrorKind>> {
        Ok(self
            .repository
            .get_by_ids(ids, db_connection)
            .await?
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect())
    }

    /// Gets the [`Resources`][Vec<Resource>] with the provided IDs.
    pub async fn get_by_names<C: ConnectionTrait>(
        &self,
        names: &[String],
        db_connection: &C,
    ) -> Result<Vec<Resource>, DomainError<ResourceErrorKind>> {
        Ok(self
            .repository
            .get_by_names(names, db_connection)
            .await?
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect())
    }
}
