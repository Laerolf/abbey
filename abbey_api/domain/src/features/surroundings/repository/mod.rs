use entity::{
    cyclic_process_resources, cyclic_processes, resources, sources, surroundings,
    surroundings_sources,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter,
};

use crate::{
    features::{
        output::{domain::resource::Resource, mapper::ResourceMapper},
        process::{
            domain::{Process, cyclic_process::CyclicProcess},
            mapper::cyclic_process::CyclicProcessMapper,
        },
        source::mapper::SourceMapper,
        surroundings::{
            domain::Surroundings,
            error::SurroundingsErrorKind,
            forms::{SurroundingSourceCreationForm, SurroundingsCreationForm},
            mapper::{SurroundingsMapper, SurroundingsSourceMapper},
        },
    },
    shared::error::DomainError,
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
                DomainError::from(SurroundingsErrorKind::FindAllSources).with_cause(error)
            })?;

        let source_process_ids: Vec<i32> = source_models
            .iter()
            .map(|model| model.cyclic_process_id)
            .collect();

        let all_source_output_resource_assignments = cyclic_process_resources::Entity::find()
            .filter(
                cyclic_process_resources::Column::CylicProcessId.is_in(source_process_ids.clone()),
            )
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::FindById).with_cause(error)
            })?;

        let all_source_output_resource_ids: Vec<i32> = all_source_output_resource_assignments
            .iter()
            .map(|model| model.resource_id)
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
                DomainError::from(SurroundingsErrorKind::FindAllSources).with_cause(error)
            })?
            .into_iter()
            .map(|cyclic_process_model| {
                let output_resource_ids: Vec<i32> = all_source_output_resource_assignments
                    .iter()
                    .filter(|model| model.cylic_process_id == cyclic_process_model.id)
                    .map(|model| model.resource_id)
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
                    .find(|model| source_model.cyclic_process_id == model.id().unwrap())
                    .ok_or(DomainError::from(SurroundingsErrorKind::FindAllSources))?;

                Ok(SourceMapper::to_domain_entity(
                    source_model,
                    process_model.clone(),
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;

        SurroundingsMapper::to_domain_entity(surroundings_model, sources).map_err(|error| {
            DomainError::from(SurroundingsErrorKind::FindAllSources).with_cause(error)
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
    pub async fn create(
        &self,
        creation_form: SurroundingsCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Surroundings, DomainError<SurroundingsErrorKind>> {
        let new_surroundings_model: surroundings::Model = SurroundingsMapper::to_new_active_model()
            .insert(db_transaction)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::Creation).with_cause(error)
            })?;

        if !creation_form.source_ids.is_empty() {
            let surroundings_sources: Vec<surroundings_sources::ActiveModel> = creation_form
                .source_ids
                .iter()
                .map(|source_id| {
                    SurroundingsSourceMapper::to_new_active_model(
                        SurroundingSourceCreationForm::new(new_surroundings_model.id, *source_id),
                    )
                })
                .collect();

            surroundings_sources::Entity::insert_many(surroundings_sources)
                .exec(db_transaction)
                .await
                .map_err(|error| {
                    DomainError::from(SurroundingsErrorKind::Creation).with_cause(error)
                })?;
        }

        self.find_by_id_with_relations(&new_surroundings_model.id, db_transaction)
            .await?
            .ok_or(DomainError::from(SurroundingsErrorKind::Creation))
    }
}
