use sea_orm::DatabaseTransaction;
use time::Duration;

use crate::{
    features::{
        output::{
            domain::resource::{Category, Resource},
            service::ResourceService,
        },
        process::{domain::CyclicProcess, service::CyclicProcessService},
        source::{domain::Source, error::SourceErrorKind, service::SourceService},
        surroundings::{
            error::SurroundingsErrorKind,
            forms::SurroundingsCreationForm,
            repository::{SurroundingsRepository, SurroundingsWithRelations},
        },
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Surroundings`] topic.
#[derive(Clone)]
pub struct SurroundingsService {
    repository: SurroundingsRepository,
    source_service: SourceService,
    cyclic_process_service: CyclicProcessService,
    resource_service: ResourceService,
}

impl SurroundingsService {
    /// Creates a new [`SurroundingsService`].
    pub fn new(
        source_service: SourceService,
        cyclic_process_service: CyclicProcessService,
        resource_service: ResourceService,
    ) -> Self {
        Self {
            repository: SurroundingsRepository::default(),
            source_service,
            cyclic_process_service,
            resource_service,
        }
    }

    /// Creates the [Sources][`Source`] for a new [`Surroundings`].
    async fn create_sources_in_transaction(
        &self,
        transaction: &DatabaseTransaction,
    ) -> Result<Vec<Source>, DomainError<SourceErrorKind>> {
        let beach_resources: Vec<Resource> = vec![
            self.resource_service
                .find_by_name_or_create_in_transaction("sand", Category::Material, transaction)
                .await
                .map_err(|error| DomainError::from(SourceErrorKind::Creation).with_cause(error))?,
            self.resource_service
                .find_by_name_or_create_in_transaction("seaweed", Category::Material, transaction)
                .await
                .map_err(|error| DomainError::from(SourceErrorKind::Creation).with_cause(error))?,
        ];

        let beach_process: CyclicProcess = self
            .cyclic_process_service
            .create_cyclic_process_in_transaction(
                beach_resources,
                Duration::minutes(1),
                transaction,
            )
            .await
            .map_err(|error| DomainError::from(SourceErrorKind::Creation).with_cause(error))?;

        let beach: Source = self
            .source_service
            .create_source_in_transaction("the_beach", beach_process, transaction)
            .await?;

        Ok(vec![beach])
    }

    /// Creates a new [`Surroundings`][`SurroundingsWithRelations`].
    pub async fn create_surroundings_in_transaction(
        &self,
        transaction: &DatabaseTransaction,
    ) -> Result<SurroundingsWithRelations, DomainError<SurroundingsErrorKind>> {
        let source_ids: Vec<i32> = self
            .create_sources_in_transaction(transaction)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::Creation).with_cause(error))?
            .iter()
            .map(|source| source.id)
            .collect();

        let creation_form = SurroundingsCreationForm::new(source_ids);

        self.repository
            .create_with_relations_in_transaction(creation_form, transaction)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::Creation).with_cause(error))
    }
}
