use entity::{cyclic_processes, sources, surroundings, surroundings_sources};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, ModelTrait, QueryFilter,
};

use crate::{
    features::{
        process::mapper::CyclicProcessMapper,
        source::{domain::Source, mapper::SourceMapper},
        surroundings::{forms::SurroundingsCreationForm, mapper::SurroundingsMapper},
    },
    shared::db::DatabaseClient,
};

/// Represents an element that handles all [Surroundings][`super::domain::Surroundings`] database topics.
#[derive(Default, Clone)]
pub struct SurroundingsRepository {}

impl SurroundingsRepository {
    /// Inserts a [Surroundings][`surroundings::Model`].
    pub async fn insert(
        &self,
        new_surroundings: surroundings::ActiveModel,
    ) -> Result<surroundings::Model, DbErr> {
        new_surroundings.insert(DatabaseClient::get_connection()).await
    }

    /// Finds a [Player][`surroundings::Model`] by its ID.
    pub async fn find_by_id(&self, id: i32) -> Result<Option<surroundings::Model>, DbErr> {
        surroundings::Entity::find_by_id(id)
            .one(DatabaseClient::get_connection())
            .await
    }

    /// Finds a [Player][`surroundings::Model`] by its ID.
    pub async fn find_by_id_in_transaction(
        &self,
        id: i32,
        transaction: &DatabaseTransaction,
    ) -> Result<Option<surroundings::Model>, DbErr> {
        surroundings::Entity::find_by_id(id).one(transaction).await
    }

    /// Finds the [sources][`Source`] of the [`Surroundings`][`surroundings::Model`].
    async fn find_sources(
        &self,
        surroundings_source_models: Vec<surroundings_sources::Model>,
    ) -> Result<Vec<Source>, DbErr> {
        let db = DatabaseClient::get_connection();

        let surroundings_source_ids: Vec<i32> = surroundings_source_models
            .iter()
            .map(|source| source.id)
            .collect();

        let source_models = sources::Entity::find()
            .filter(sources::Column::Id.is_in(surroundings_source_ids))
            .all(db)
            .await?;

        if source_models.is_empty() {
            return Ok(Vec::new());
        }

        let source_process_ids: Vec<i32> = source_models
            .iter()
            .map(|source| source.cyclic_process_id)
            .collect();

        let source_processes = cyclic_processes::Entity::find()
            .filter(cyclic_processes::Column::Id.is_in(source_process_ids))
            .all(db)
            .await?;

        Ok(source_models
            .into_iter()
            .map(|source_model| {
                let process_model = source_processes
                    .iter()
                    .find(|process| process.id == source_model.cyclic_process_id)
                    .expect("A source should have a process.")
                    .clone();

                SourceMapper::to_domain_entity(
                    source_model,
                    CyclicProcessMapper::to_domain_entity(process_model),
                )
            })
            .collect())
    }

    /// Finds a [Surroundings][`SurroundingsWithRelations`] by its ID and with all its related entities.
    pub async fn find_by_id_with_relations(
        &self,
        id: i32,
    ) -> Result<Option<SurroundingsWithRelations>, DbErr> {
        let Some(surroundings_model) = self.find_by_id(id).await? else {
            return Ok(None);
        };

        let db = DatabaseClient::get_connection();

        let surroundings_sources = surroundings_model
            .find_related(surroundings_sources::Entity)
            .all(db)
            .await?;

        if surroundings_sources.is_empty() {
            return Ok(Some(SurroundingsWithRelations {
                surroundings: surroundings_model,
                sources: Vec::new(),
            }));
        }

        let sources = self.find_sources(surroundings_sources).await?;

        Ok(Some(SurroundingsWithRelations {
            surroundings: surroundings_model,
            sources,
        }))
    }

    /// Finds a [Surroundings][`SurroundingsWithRelations`] by its ID and with all its related entities.
    pub async fn find_by_id_with_relations_in_transaction(
        &self,
        id: i32,
        transaction: &DatabaseTransaction,
    ) -> Result<Option<SurroundingsWithRelations>, DbErr> {
        let Some(surroundings_model) = self.find_by_id_in_transaction(id, transaction).await?
        else {
            return Ok(None);
        };

        let surroundings_sources = surroundings_model
            .find_related(surroundings_sources::Entity)
            .all(transaction)
            .await?;

        if surroundings_sources.is_empty() {
            return Ok(Some(SurroundingsWithRelations {
                surroundings: surroundings_model,
                sources: Vec::new(),
            }));
        }

        let sources = self.find_sources(surroundings_sources).await?;

        Ok(Some(SurroundingsWithRelations {
            surroundings: surroundings_model,
            sources,
        }))
    }

    /// Creates a [`Surroundings`][`SurroundingsWithRelations`] with all its relations.
    pub async fn create_with_relations_in_transaction(
        &self,
        form: SurroundingsCreationForm,
        transaction: &DatabaseTransaction,
    ) -> Result<SurroundingsWithRelations, DbErr> {
        let surroundings = SurroundingsMapper::to_new_active_model(form.clone())
            .insert(transaction)
            .await?;

        if !form.source_ids.is_empty() {
            let surroundings_sources: Vec<surroundings_sources::ActiveModel> = form
                .source_ids
                .iter()
                .map(|id| surroundings_sources::ActiveModel {
                    id: NotSet,
                    surroundings_id: Set(surroundings.id),
                    source_id: Set(*id),
                })
                .collect();

            surroundings_sources::Entity::insert_many(surroundings_sources)
                .exec(transaction)
                .await?;
        }

        self.find_by_id_with_relations_in_transaction(surroundings.id, transaction)
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Failed to find new created surroundings.".to_string(),
            ))
    }
}

/// [`Surroundings`][`surroundings::Model`] with all its related entities.
pub struct SurroundingsWithRelations {
    pub surroundings: surroundings::Model,
    pub sources: Vec<Source>,
}
