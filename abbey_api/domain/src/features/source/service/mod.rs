use sea_orm::DatabaseTransaction;

use crate::{
    features::{
        process::domain::{CyclicProcess, Process},
        source::{
            domain::Source, error::SourceErrorKind, forms::SourceCreationForm,
            mapper::SourceMapper, repository::SourceRepository,
        },
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Source`] topic.
#[derive(Default, Clone)]
pub struct SourceService {
    repository: SourceRepository,
}

impl SourceService {
    /// Creates a new [`Source`].
    pub async fn create_source_in_transaction(
        &self,
        name: impl Into<String>,
        process: CyclicProcess,
        transaction: &DatabaseTransaction,
    ) -> Result<Source, DomainError<SourceErrorKind>> {
        let creation_form = SourceCreationForm::new(name, process.id());

        let related_source = self
            .repository
            .create_with_relations_in_transaction(creation_form, transaction)
            .await
            .map_err(|error| DomainError::from(SourceErrorKind::Creation).with_cause(error))?;

        Ok(SourceMapper::to_domain_entity_with_relations(
            related_source,
        ))
    }
}
