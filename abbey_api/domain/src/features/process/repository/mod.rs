use entity::{cyclic_process_resources, cyclic_processes, resources};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, ModelTrait, QueryFilter,
};

use crate::{
    features::process::{forms::CyclicProcessCreationForm, mapper::CyclicProcessMapper},
    shared::db::DatabaseClient,
};

/// [`CyclicProcess`][`cyclic_processes::Model`] with all its related entities.
pub struct CyclicProcessWithRelations {
    pub cyclic_process: cyclic_processes::Model,
    pub resources: Vec<resources::Model>,
}

/// Represents an element that handles all [CyclicProcess][`super::domain::CyclicProcess`] database topics.
#[derive(Default, Clone)]
pub struct CyclicProcessRepository {}

impl CyclicProcessRepository {
    /// Inserts a [CyclicProcess][`cyclic_processes::Model`].
    pub async fn insert(
        &self,
        new_cyclic_process: cyclic_processes::ActiveModel,
    ) -> Result<cyclic_processes::Model, DbErr> {
        new_cyclic_process
            .insert(DatabaseClient::get_connection())
            .await
    }

    /// Finds a [CyclicProcess][`cyclic_processes::Model`] by its ID.
    pub async fn find_by_id(&self, id: i32) -> Result<Option<cyclic_processes::Model>, DbErr> {
        cyclic_processes::Entity::find_by_id(id)
            .one(DatabaseClient::get_connection())
            .await
    }

    /// Finds a [CyclicProcess][`cyclic_processes::Model`] by its ID.
    pub async fn find_by_id_in_transaction(
        &self,
        id: i32,
        transaction: &DatabaseTransaction,
    ) -> Result<Option<cyclic_processes::Model>, DbErr> {
        cyclic_processes::Entity::find_by_id(id)
            .one(transaction)
            .await
    }

    /// Finds a [CyclicProcess][`CyclicProcessWithRelations`] with all its related entities by its ID.
    pub async fn find_by_id_with_relations(
        &self,
        id: i32,
    ) -> Result<Option<CyclicProcessWithRelations>, DbErr> {
        let Some(model) = self.find_by_id(id).await? else {
            return Ok(None);
        };

        let db = DatabaseClient::get_connection();

        let process_resources = model
            .find_related(cyclic_process_resources::Entity)
            .all(db)
            .await?;

        let resource_ids: Vec<i32> = process_resources.iter().map(|pr| pr.resource_id).collect();

        let resources = resources::Entity::find()
            .filter(resources::Column::Id.is_in(resource_ids))
            .all(db)
            .await?;

        Ok(Some(CyclicProcessWithRelations {
            cyclic_process: model,
            resources,
        }))
    }

    /// Finds a [CyclicProcess][`CyclicProcessWithRelations`] with all its related entities by its ID.
    pub async fn find_by_id_with_relations_in_transaction(
        &self,
        id: i32,
        transaction: &DatabaseTransaction,
    ) -> Result<Option<CyclicProcessWithRelations>, DbErr> {
        let Some(model) = self.find_by_id_in_transaction(id, transaction).await? else {
            return Ok(None);
        };

        let process_resources = model
            .find_related(cyclic_process_resources::Entity)
            .all(transaction)
            .await?;

        let resource_ids: Vec<i32> = process_resources.iter().map(|pr| pr.resource_id).collect();

        let resources = resources::Entity::find()
            .filter(resources::Column::Id.is_in(resource_ids))
            .all(transaction)
            .await?;

        Ok(Some(CyclicProcessWithRelations {
            cyclic_process: model,
            resources,
        }))
    }

    /// Creates a [`CyclicProcess`][`CyclicProcessWithRelations`] with all its relations.
    pub async fn create_with_relations_in_transaction(
        &self,
        form: CyclicProcessCreationForm,
        transaction: &DatabaseTransaction,
    ) -> Result<CyclicProcessWithRelations, DbErr> {
        let process = CyclicProcessMapper::to_new_active_model(form.clone())
            .insert(transaction)
            .await?;

        if !form.output_resources_ids.is_empty() {
            let process_resources: Vec<cyclic_process_resources::ActiveModel> = form
                .output_resources_ids
                .iter()
                .map(|id| cyclic_process_resources::ActiveModel {
                    id: NotSet,
                    cylic_process_id: Set(process.id),
                    resource_id: Set(*id),
                })
                .collect();

            cyclic_process_resources::Entity::insert_many(process_resources)
                .exec(transaction)
                .await?;
        }

        self.find_by_id_with_relations_in_transaction(process.id, transaction)
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Failed to find a new created process".to_string(),
            ))
    }
}
