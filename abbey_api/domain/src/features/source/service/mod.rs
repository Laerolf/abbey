use sea_orm::DatabaseTransaction;

use crate::{
    features::{
        process::domain::{Process, cyclic_process::CyclicProcess},
        source::{
            domain::Source, error::SourceErrorKind, forms::SourceCreationForm,
            repository::SourceRepository,
        },
    },
    shared::error::DomainError,
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
