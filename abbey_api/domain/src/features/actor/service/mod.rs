use sea_orm::DatabaseTransaction;

use crate::{
    features::actor::{
        domain::monk::Monk, error::ActorErrorKind, forms::MonkCreationForm, mapper::MonkMapper,
        repository::MonkRepository,
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Monk`] topic.
#[derive(Clone, Default)]
pub struct MonkService {
    repository: MonkRepository,
}

impl MonkService {
    /// Creates a new [`Monk`].
    pub async fn create_monk(
        &self,
        creation_form: MonkCreationForm,
    ) -> Result<Monk, DomainError<ActorErrorKind>> {
        let related_monk = self
            .repository
            .create_with_relations(creation_form)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Creation).with_cause(error))?;

        Ok(MonkMapper::to_domain_entity_with_relations(related_monk))
    }

    /// Creates new [`Monks`][`Monk`].
    pub async fn create_many_monks_in_transaction(
        &self,
        creation_forms: Vec<MonkCreationForm>,
        transaction: &DatabaseTransaction,
    ) -> Result<Vec<Monk>, DomainError<ActorErrorKind>> {
        let related_monks = self
            .repository
            .create_many_with_relations_in_transaction(creation_forms, transaction)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Creation).with_cause(error))?;

        Ok(related_monks
            .into_iter()
            .map(MonkMapper::to_domain_entity_with_relations)
            .collect())
    }
}
