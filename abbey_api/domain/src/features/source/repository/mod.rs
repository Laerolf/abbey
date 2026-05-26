use entity::{cyclic_process_resources, cyclic_processes, resources, sources};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter,
    QueryOrder,
};

use crate::{
    features::{
        output::{domain::resource::Resource, mapper::ResourceMapper},
        process::mapper::cyclic_process::CyclicProcessMapper,
        source::{
            domain::Source, error::SourceErrorKind, forms::SourceCreationForm, mapper::SourceMapper,
        },
    },
    shared::error::DomainError,
};

/// Represents an element that handles all [Source][`super::domain::Source`] database topics.
#[derive(Default, Clone)]
pub struct SourceRepository;

impl SourceRepository {
    /// Gets all relations of a [`Source`].
    async fn get_relations<C: ConnectionTrait>(
        &self,
        source_model: sources::Model,
        db_connection: &C,
    ) -> Result<Source, DomainError<SourceErrorKind>> {
        let source_process_model = cyclic_processes::Entity::find()
            .inner_join(sources::Entity)
            .filter(sources::Column::CyclicProcessId.eq(source_model.cyclic_process_id))
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(SourceErrorKind::FindProcess).with_cause(error))?
            .ok_or(DomainError::from(SourceErrorKind::ProcessNotFound))?;

        let source_process_output_resources: Vec<Resource> = resources::Entity::find()
            .inner_join(cyclic_process_resources::Entity)
            .filter(cyclic_process_resources::Column::CyclicProcessId.eq(source_process_model.id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SourceErrorKind::NoPossibleResources).with_cause(error)
            })?
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect();

        let source_process = CyclicProcessMapper::to_domain_entity(
            source_process_model,
            source_process_output_resources,
            Vec::new(),
        )
        .map_err(|error| DomainError::from(SourceErrorKind::ProcessNotFound).with_cause(error))?;

        Ok(SourceMapper::to_domain_entity(source_model, source_process))
    }

    /// Gets [`Sources`][Vec<sources::Model>] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<sources::Model>, DomainError<SourceErrorKind>> {
        sources::Entity::find()
            .filter(sources::Column::Id.is_in(ids.to_vec()))
            .order_by_asc(sources::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(SourceErrorKind::GetByIds).with_cause(error))
    }

    /// Finds a [`Source`] by its ID.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<Source>, DomainError<SourceErrorKind>> {
        let Some(source_model) = sources::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(SourceErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        Ok(Some(self.get_relations(source_model, db_connection).await?))
    }

    /// Creates a [`Source`] and persists it in the database.
    pub async fn create(
        &self,
        creation_form: SourceCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Source, DomainError<SourceErrorKind>> {
        let new_source_model: sources::Model = SourceMapper::to_new_active_model(creation_form)
            .insert(db_transaction)
            .await
            .map_err(|error| DomainError::from(SourceErrorKind::Creation).with_cause(error))?;

        self.find_by_id_with_relations(&new_source_model.id, db_transaction)
            .await?
            .ok_or(DomainError::from(SourceErrorKind::Creation))
    }
}
