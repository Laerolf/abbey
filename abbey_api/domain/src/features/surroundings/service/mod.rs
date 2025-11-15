use time::Duration;

use crate::{
    features::{
        output::{
            domain::resource::{Category, Resource},
            service::ResourceService,
        },
        process::{domain::CyclicProcess, service::CyclicProcessService},
        source::{domain::Source, service::SourceService},
        surroundings::{
            domain::Surroundings, error::SurroundingsError, forms::SurroundingsCreationForm,
            mapper::SurroundingsMapper, repository::SurroundingsRepository,
        },
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Surroundings`] topic.
#[derive(Default)]
pub struct SurroundingsService {
    repository: SurroundingsRepository,
    source_service: SourceService,
    cyclic_process_service: CyclicProcessService,
    resource_service: ResourceService,
}

impl SurroundingsService {
    /// Creates the [Sources][`Source`] for a new [`Surroundings`].
    async fn create_sources(&self) -> Result<Vec<Source>, Box<dyn DomainError>> {
        let beach_resources: Vec<Resource> = vec![
            self.resource_service
                .find_by_name_or_create("sand", Category::Material)
                .await?,
            self.resource_service
                .find_by_name_or_create("seaweed", Category::Material)
                .await?,
        ];
        let beach_process: CyclicProcess = self
            .cyclic_process_service
            .create_cyclic_process(beach_resources, Duration::minutes(1), Vec::new())
            .await?;

        let beach: Source = self
            .source_service
            .create_source("the_beach", beach_process)
            .await?;

        Ok(vec![beach])
    }

    /// Creates a new [`Surroundings`].
    pub async fn create_surroundings(&self) -> Result<Surroundings, Box<dyn DomainError>> {
        let sources = self
            .create_sources()
            .await
            .expect("Failed to create Sources for a Surrounding.");

        let creation_form =
            SurroundingsCreationForm::new(sources.iter().map(|source| source.id).collect());

        match self
            .repository
            .insert(SurroundingsMapper::to_new_active_model(creation_form))
            .await
        {
            Ok(new_surroundings) => Ok(SurroundingsMapper::to_domain_entity(
                new_surroundings,
                sources,
            )),
            Err(_error) => Err(Box::new(SurroundingsError::Creation)),
        }
    }
}
