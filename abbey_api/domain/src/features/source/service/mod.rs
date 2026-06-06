use sea_orm::ConnectionTrait;

use crate::{
    features::{
        process::service::cyclic_process::CyclicProcessQueryService,
        source::{
            domain::Source, error::SourceErrorKind, forms::SourceBlueprint, mapper::SourceMapper,
            repository::SourceRepository,
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a command service for [`Sources`][Source].
#[derive(Clone)]
pub struct SourceCommandService {
    repository: SourceRepository,
    source_query_service: SourceQueryService,
}

impl SourceCommandService {
    /// Creates a new [`SourceCommandService`].
    pub fn new(repository: SourceRepository, source_query_service: SourceQueryService) -> Self {
        Self {
            repository,
            source_query_service,
        }
    }

    /// Creates new [`Sources`][Vec<Source>].
    pub async fn create_many<C: ConnectionTrait>(
        &self,
        blueprints: Vec<SourceBlueprint>,
        db_connection: &C,
    ) -> Result<Vec<Source>, DomainError<SourceErrorKind>> {
        let active_models = blueprints
            .into_iter()
            .map(SourceMapper::to_new_active_model)
            .collect();

        let models = self
            .repository
            .create_many(active_models, db_connection)
            .await?;

        let ids: Vec<i32> = models.iter().map(|model| model.id).collect();

        self.source_query_service
            .get_by_ids(&ids, db_connection)
            .await
            .map_err(|error| DomainError::from(SourceErrorKind::Creation).with_cause(error))
    }
}

/// Represents a query service for [`Sources`][Source].
#[derive(Clone)]
pub struct SourceQueryService {
    repository: SourceRepository,
    cyclic_process_query_service: CyclicProcessQueryService,
}

impl SourceQueryService {
    /// Creates a new [`SourceQueryService`].
    pub fn new(
        repository: SourceRepository,
        cyclic_process_query_service: CyclicProcessQueryService,
    ) -> Self {
        Self {
            repository,
            cyclic_process_query_service,
        }
    }

    /// Gets [Sources][Vec<Source>] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<Source>, DomainError<SourceErrorKind>> {
        let models = self.repository.get_by_ids(ids, db_connection).await?;

        let process_ids: Vec<i32> = models.iter().map(|model| model.cyclic_process_id).collect();

        let processes = self
            .cyclic_process_query_service
            .get_by_ids(&process_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SourceErrorKind::ProcessNotFound).with_cause(error)
            })?;

        models
            .into_iter()
            .map(|source_model| {
                let process = processes
                    .iter()
                    .find(|process| process.id().unwrap() == source_model.cyclic_process_id)
                    .ok_or(DomainError::from(SourceErrorKind::FindProcess))
                    .map_err(|error| {
                        DomainError::from(SourceErrorKind::ProcessNotFound).with_cause(error)
                    })?;

                Ok(SourceMapper::to_domain_entity(
                    source_model,
                    process.clone(),
                ))
            })
            .collect::<Result<Vec<Source>, DomainError<SourceErrorKind>>>()
    }
}
