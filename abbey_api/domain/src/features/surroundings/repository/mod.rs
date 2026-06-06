use entity::{
    cyclic_process_resources, cyclic_processes, resources, sources, surroundings,
    surroundings_sources,
};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use tracing::warn;

use crate::{
    features::{
        output::{domain::resource::Resource, mapper::ResourceMapper},
        process::{
            domain::cyclic_process::CyclicProcess, mapper::cyclic_process::CyclicProcessMapper,
        },
        source::mapper::SourceMapper,
        surroundings::{
            domain::Surroundings, error::SurroundingsErrorKind, mapper::SurroundingsMapper,
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents an element that handles all [Surroundings][`super::domain::Surroundings`] database topics.
#[derive(Default, Clone)]
pub struct SurroundingsRepository;

impl SurroundingsRepository {
    /// Get all relations of a [`Surroundings`].
    async fn get_relations<C: ConnectionTrait>(
        &self,
        surroundings_model: surroundings::Model,
        db_connection: &C,
    ) -> Result<Surroundings, DomainError<SurroundingsErrorKind>> {
        let source_models = sources::Entity::find()
            .inner_join(surroundings_sources::Entity)
            .filter(surroundings_sources::Column::SourceId.eq(surroundings_model.id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::GetAllSources).with_cause(error)
            })?;

        let source_process_ids: Vec<i32> = source_models
            .iter()
            .map(|source_model| source_model.cyclic_process_id)
            .collect();

        let all_source_output_resource_assignments = cyclic_process_resources::Entity::find()
            .filter(
                cyclic_process_resources::Column::CyclicProcessId.is_in(source_process_ids.clone()),
            )
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::FindById).with_cause(error)
            })?;

        let all_source_output_resource_ids: Vec<i32> = all_source_output_resource_assignments
            .iter()
            .map(|source_output_resource_assignment_model| {
                source_output_resource_assignment_model.resource_id
            })
            .collect();

        let all_source_output_resources: Vec<Resource> = resources::Entity::find()
            .filter(resources::Column::Id.is_in(all_source_output_resource_ids))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::FindById).with_cause(error))?
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect();

        let all_source_cyclic_processes: Vec<CyclicProcess> = cyclic_processes::Entity::find()
            .filter(cyclic_processes::Column::Id.is_in(source_process_ids))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::GetAllSources).with_cause(error)
            })?
            .into_iter()
            .map(|cyclic_process_model| {
                let output_resource_ids: Vec<i32> = all_source_output_resource_assignments
                    .iter()
                    .filter(|source_output_resource_assignment_model| {
                        source_output_resource_assignment_model.cyclic_process_id
                            == cyclic_process_model.id
                    })
                    .map(|source_output_resource_assignment_model| {
                        source_output_resource_assignment_model.resource_id
                    })
                    .collect();

                let output_resources: Vec<Resource> = all_source_output_resources
                    .clone()
                    .into_iter()
                    .filter(|resource| output_resource_ids.contains(&resource.id().unwrap()))
                    .collect();

                CyclicProcessMapper::to_domain_entity(
                    cyclic_process_model,
                    output_resources,
                    Vec::new(),
                )
                .unwrap()
            })
            .collect();

        let sources = source_models
            .into_iter()
            .map(|source_model| {
                let process_model = all_source_cyclic_processes
                    .iter()
                    .find(|cyclic_process_model| {
                        source_model.cyclic_process_id == cyclic_process_model.id().unwrap()
                    })
                    .ok_or(DomainError::from(SurroundingsErrorKind::GetAllSources))?;

                Ok(SourceMapper::to_domain_entity(
                    source_model,
                    process_model.clone(),
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;

        SurroundingsMapper::to_domain_entity(surroundings_model, sources).map_err(|error| {
            DomainError::from(SurroundingsErrorKind::GetAllSources).with_cause(error)
        })
    }

    /// Finds a [Surroundings][`surroundings::Model`] by its ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<surroundings::Model>, DomainError<SurroundingsErrorKind>> {
        surroundings::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::FindById).with_cause(error))
    }

    /// Gets the [Surroundings][`Vec<surroundings::Model>`] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<surroundings::Model>, DomainError<SurroundingsErrorKind>> {
        surroundings::Entity::find()
            .filter(surroundings::Column::Id.is_in(ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::GetByIds).with_cause(error))
    }

    /// Gets all the [Sources][Vec<Source>] for the provided Surroundings ID.
    pub async fn get_all_sources_by_surroundings_id<C: ConnectionTrait>(
        &self,
        surroundings_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<surroundings_sources::Model>, DomainError<SurroundingsErrorKind>> {
        surroundings_sources::Entity::find()
            .filter(surroundings_sources::Column::SurroundingsId.eq(*surroundings_id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::GetAllSources).with_cause(error)
            })
    }

    /// Gets all the [Sources][Vec<Source>] for the provided Surroundings IDs.
    pub async fn get_all_sources_by_surroundings_ids<C: ConnectionTrait>(
        &self,
        surroundings_ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<surroundings_sources::Model>, DomainError<SurroundingsErrorKind>> {
        surroundings_sources::Entity::find()
            .filter(surroundings_sources::Column::SurroundingsId.is_in(surroundings_ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::GetAllSources).with_cause(error)
            })
    }

    /// Finds a [Surroundings] by its ID and with all its related entities.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<Surroundings>, DomainError<SurroundingsErrorKind>> {
        let Some(surroundings_model) = surroundings::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::FindById).with_cause(error)
            })?
        else {
            return Ok(None);
        };

        Ok(Some(
            self.get_relations(surroundings_model, db_connection)
                .await?,
        ))
    }

    /// Creates a new [`Surroundings`] and persists it in the database.
    pub async fn create<C: ConnectionTrait>(
        &self,
        model: surroundings::ActiveModel,
        db_connection: &C,
    ) -> Result<surroundings::Model, DomainError<SurroundingsErrorKind>> {
        surroundings::Entity::insert(model)
            .exec_with_returning(db_connection)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::Creation).with_cause(error))
    }

    /// Assigns Sources to a [`Surroundings`].
    pub async fn assign_sources_to_surroundings<C: ConnectionTrait>(
        &self,
        models: Vec<surroundings_sources::ActiveModel>,
        db_connection: &C,
    ) -> Result<Vec<surroundings_sources::Model>, DomainError<SurroundingsErrorKind>> {
        if models.is_empty() {
            warn!("Skipping this insertion because no models were provided.");
            return Ok(vec![]);
        }

        surroundings_sources::Entity::insert_many(models)
            .exec_with_returning_many(db_connection)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::Creation).with_cause(error))
    }
}
