use sea_orm::DatabaseTransaction;
use time::Duration;

use crate::{
    features::{
        output::domain::resource::Resource,
        process::{
            domain::CyclicProcess, error::ProcessErrorKind, forms::CyclicProcessCreationForm,
            mapper::CyclicProcessMapper, repository::CyclicProcessRepository,
        },
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`CyclicProcess`] topic.
#[derive(Default, Clone)]
pub struct CyclicProcessService {
    repository: CyclicProcessRepository,
}

impl CyclicProcessService {
    /// Finds a [`CyclicProcess`] and all its related entities by its ID.
    pub async fn find_by_id_with_relations(
        &self,
        id: i32,
    ) -> Result<Option<CyclicProcess>, ProcessErrorKind> {
        let Some(model) = self
            .repository
            .find_by_id_with_relations(id)
            .await
            .map_err(|_error| ProcessErrorKind::Creation)?
        else {
            return Ok(None);
        };

        Ok(Some(CyclicProcessMapper::to_domain_entity_with_relations(
            model,
        )))
    }

    /// Creates a new [`CyclicProcess`].
    pub async fn create_cyclic_process_in_transaction(
        &self,
        output_resources: Vec<Resource>,
        cycle_interval: Duration,
        transaction: &DatabaseTransaction,
    ) -> Result<CyclicProcess, DomainError<ProcessErrorKind>> {
        let creation_form = CyclicProcessCreationForm::new(
            output_resources
                .iter()
                .map(|resource| resource.id)
                .collect(),
            cycle_interval,
        );

        let related_process = self
            .repository
            .create_with_relations_in_transaction(creation_form, transaction)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))?;

        Ok(CyclicProcessMapper::to_domain_entity_with_relations(
            related_process,
        ))
    }
}
