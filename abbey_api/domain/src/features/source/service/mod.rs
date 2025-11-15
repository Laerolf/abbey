use crate::{
    features::{
        process::domain::{CyclicProcess, Process},
        source::{
            domain::Source, error::SourceError, forms::SourceCreationForm, mapper::SourceMapper,
            repository::SourceRepository,
        },
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Source`] topic.
#[derive(Default)]
pub struct SourceService {
    repository: SourceRepository,
}

impl SourceService {
    /// Creates a new [`Source`].
    pub async fn create_source(
        &self,
        name: impl Into<String>,
        process: CyclicProcess,
    ) -> Result<Source, Box<dyn DomainError>> {
        let creation_form = SourceCreationForm::new(name, process.id());

        match self
            .repository
            .insert(SourceMapper::to_new_active_model(creation_form))
            .await
        {
            Ok(new_source) => Ok(SourceMapper::to_domain_entity(new_source, process)),
            Err(_error) => Err(Box::new(SourceError::Creation)),
        }
    }
}
