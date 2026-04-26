use sea_orm::DatabaseTransaction;

use crate::{
    features::actor::{
        domain::monk::Monk, error::ActorErrorKind, forms::MonkCreationForm,
        repository::MonkRepository,
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Monk`] topic.
#[derive(Clone)]
pub struct MonkService {
    repository: MonkRepository,
}

impl MonkService {
    /// Creates a new [`MonkService`].
    pub fn new(repository: MonkRepository) -> Self {
        Self { repository }
    }

    /// Creates a new [`Monk`].
    pub async fn create_monk(
        &self,
        creation_form: MonkCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Monk, DomainError<ActorErrorKind>> {
        self.repository
            .create(creation_form, db_transaction)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Creation).with_cause(error))
    }

    /// Creates new [`Monks`][Vec<Monk>].
    pub async fn create_many_monks(
        &self,
        creation_forms: Vec<MonkCreationForm>,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Vec<Monk>, DomainError<ActorErrorKind>> {
        self.repository
            .create_many(creation_forms, db_transaction)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Creation).with_cause(error))
    }
}
