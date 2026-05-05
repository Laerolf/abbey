use sea_orm::DatabaseTransaction;
use time::Duration;

use crate::{
    features::{
        output::{
            domain::resource::{Category, Resource},
            service::ResourceService,
        },
        process::{
            domain::cyclic_process::CyclicProcess, service::cyclic_process::CyclicProcessService,
        },
        source::{domain::Source, error::SourceErrorKind, service::SourceService},
        surroundings::{
            domain::Surroundings, error::SurroundingsErrorKind, forms::SurroundingsCreationForm,
            repository::SurroundingsRepository,
        },
    },
    shared::{DomainElement, error::DomainError},
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
        repository: SurroundingsRepository,
        source_service: SourceService,
        cyclic_process_service: CyclicProcessService,
        resource_service: ResourceService,
    ) -> Self {
        Self {
            repository,
            source_service,
            cyclic_process_service,
            resource_service,
        }
    }

    /// Creates the [Sources][Vec<Source>] for a new [`Surroundings`].
    async fn create_sources(
        &self,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Vec<Source>, DomainError<SourceErrorKind>> {
        let beach_resources: Vec<Resource> = vec![
            self.resource_service
                .find_by_name_or_create("sand", Category::Material, db_transaction)
                .await
                .map_err(|error| DomainError::from(SourceErrorKind::Creation).with_cause(error))?,
            self.resource_service
                .find_by_name_or_create("seaweed", Category::Material, db_transaction)
                .await
                .map_err(|error| DomainError::from(SourceErrorKind::Creation).with_cause(error))?,
        ];

        let beach_process: CyclicProcess = self
            .cyclic_process_service
            .create(beach_resources, Duration::minutes(1), db_transaction)
            .await
            .map_err(|error| DomainError::from(SourceErrorKind::Creation).with_cause(error))?;

        let beach: Source = self
            .source_service
            .create_source("the_beach", beach_process, db_transaction)
            .await?;

        Ok(vec![beach])
    }

    /// Creates a new [`Surroundings`].
    pub async fn create_surroundings(
        &self,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Surroundings, DomainError<SurroundingsErrorKind>> {
        let source_ids: Vec<i32> = self
            .create_sources(db_transaction)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::Creation).with_cause(error))?
            .iter()
            .map(|source| source.id().unwrap())
            .collect();

        let creation_form = SurroundingsCreationForm::new(source_ids);

        self.repository.create(creation_form, db_transaction).await
    }
}
