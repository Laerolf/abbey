use entity::{cyclic_processes, sources};
use sea_orm::{ActiveModelTrait, DatabaseTransaction, DbErr, EntityTrait, ModelTrait};

use crate::{
    features::source::{forms::SourceCreationForm, mapper::SourceMapper},
    shared::db::DatabaseClient,
};

/// Represents an element that handles all [Source][`super::domain::Source`] database topics.
#[derive(Default, Clone)]
pub struct SourceRepository {}

impl SourceRepository {
    /// Inserts a [`Source`][`sources::Model`].
    pub async fn insert(&self, new_source: sources::ActiveModel) -> Result<sources::Model, DbErr> {
        new_source.insert(DatabaseClient::get_connection()).await
    }

    /// Finds a [`Source`][`sources::Model`] by its ID.
    pub async fn find_by_id(&self, id: i32) -> Result<Option<sources::Model>, DbErr> {
        sources::Entity::find_by_id(id)
            .one(DatabaseClient::get_connection())
            .await
    }

    /// Finds a [`Source`][`sources::Model`] by its ID.
    pub async fn find_by_id_in_transaction(
        &self,
        id: i32,
        transaction: &DatabaseTransaction,
    ) -> Result<Option<sources::Model>, DbErr> {
        sources::Entity::find_by_id(id).one(transaction).await
    }

    /// Finds a [`Source`][`sources::Model`] with all its related entities by its ID.
    pub async fn find_by_id_with_relations(
        &self,
        id: i32,
    ) -> Result<Option<SourceWithRelations>, DbErr> {
        let Some(model) = self.find_by_id(id).await? else {
            return Ok(None);
        };

        let db = DatabaseClient::get_connection();

        let process = model.find_related(cyclic_processes::Entity).one(db).await?;

        if process.is_none() {
            return Err(DbErr::RecordNotFound(
                "Failed to find a source's process.".to_string(),
            ));
        }

        Ok(Some(SourceWithRelations {
            source: model,
            process: process.expect("A source should have a process."),
        }))
    }

    /// Finds a [`Source`][`sources::Model`] with all its related entities by its ID.
    pub async fn find_by_id_with_relations_in_transaction(
        &self,
        id: i32,
        transaction: &DatabaseTransaction,
    ) -> Result<Option<SourceWithRelations>, DbErr> {
        let Some(model) = self.find_by_id_in_transaction(id, transaction).await? else {
            return Ok(None);
        };

        let process = model
            .find_related(cyclic_processes::Entity)
            .one(transaction)
            .await?;

        if process.is_none() {
            return Err(DbErr::RecordNotFound(
                "Failed to find a source's process.".to_string(),
            ));
        }

        Ok(Some(SourceWithRelations {
            source: model,
            process: process.expect("A source should have a process."),
        }))
    }

    /// Creates a [`Source`][`SourceWithRelations`] with all its relations.
    pub async fn create_with_relations_in_transaction(
        &self,
        form: SourceCreationForm,
        transaction: &DatabaseTransaction,
    ) -> Result<SourceWithRelations, DbErr> {
        let source = SourceMapper::to_new_active_model(form)
            .insert(transaction)
            .await?;

        self.find_by_id_with_relations_in_transaction(source.id, transaction)
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Failed to find a new created source".to_string(),
            ))
    }
}

/// A [`Source`][`sources::Model`] with all its related entities.
pub struct SourceWithRelations {
    pub source: sources::Model,
    pub process: cyclic_processes::Model,
}
