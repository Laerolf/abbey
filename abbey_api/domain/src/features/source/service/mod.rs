use sea_orm::{ConnectionTrait, DatabaseTransaction};

use crate::{
    features::{
        process::{
            domain::cyclic_process::CyclicProcess,
            service::cyclic_process::CyclicProcessQueryService,
        },
        source::{
            domain::Source, error::SourceErrorKind, forms::SourceCreationForm,
            mapper::SourceMapper, repository::SourceRepository,
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a service handling the [`Source`] topic.
#[derive(Clone)]
pub struct SourceService {
    repository: SourceRepository,
}

impl SourceService {
    /// Creates a new [`SourceService`].
    pub fn new(repository: SourceRepository) -> Self {
        Self { repository }
    }

    /// Creates a new [`Source`].
    pub async fn create_source(
        &self,
        name: impl Into<String>,
        process: CyclicProcess,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Source, DomainError<SourceErrorKind>> {
        let creation_form = SourceCreationForm::new(name, process.id().unwrap());

        self.repository.create(creation_form, db_transaction).await
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
