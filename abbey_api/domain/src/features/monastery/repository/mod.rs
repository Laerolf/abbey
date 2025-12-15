use entity::{monasteries, monastery_monks, monks};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter, TransactionTrait,
};

use crate::{
    features::monastery::{forms::MonasteryCreationForm, mapper::MonasteryMapper},
    shared::db::DatabaseClient,
};

/// [`Monastery`][`monasteries::Model`] with all its related entities.
pub struct MonasteryWithRelations {
    pub monastery: monasteries::Model,
    pub monks: Vec<monks::Model>,
}

/// Represents an element that handles all [Monastery][`super::domain::Monastery`] database topics.
#[derive(Default, Clone)]
pub struct MonasteryRepository {}

impl MonasteryRepository {
    /// Finds a [Monastery][`monasteries::Model`] by its ID.
    pub async fn find_by_id(&self, id: i32) -> Result<Option<monasteries::Model>, DbErr> {
        monasteries::Entity::find_by_id(id)
            .one(DatabaseClient::get_connection())
            .await
    }

    /// Finds a [Monastery][`monasteries::Model`] by its ID and with all its related entities.
    pub async fn find_by_id_with_relations(
        &self,
        id: i32,
    ) -> Result<Option<MonasteryWithRelations>, DbErr> {
        let Some(model) = self.find_by_id(id).await? else {
            return Ok(None);
        };

        let db = DatabaseClient::get_connection();

        let monastery_monks = model.find_related(monastery_monks::Entity).all(db).await?;

        let monk_ids: Vec<i32> = monastery_monks.iter().map(|monk| monk.id).collect();

        let monks = monks::Entity::find()
            .filter(monks::Column::Id.is_in(monk_ids))
            .all(db)
            .await?;

        Ok(Some(MonasteryWithRelations {
            monastery: model,
            monks,
        }))
    }

    /// Creates a new [`Monastery`][MonasteryWithRelations].
    pub async fn create_with_relations(
        &self,
        form: MonasteryCreationForm,
    ) -> Result<MonasteryWithRelations, DbErr> {
        let db = DatabaseClient::get_connection();

        let transaction = db.begin().await?;

        let monastery = MonasteryMapper::to_new_active_model(form.clone())
            .insert(&transaction)
            .await?;

        if !form.monk_ids.is_empty() {
            let monastery_monks: Vec<monastery_monks::ActiveModel> = form
                .monk_ids
                .iter()
                .map(|id| monastery_monks::ActiveModel {
                    id: NotSet,
                    monastery_id: Set(monastery.id),
                    monk_id: Set(*id),
                })
                .collect();

            monastery_monks::Entity::insert_many(monastery_monks)
                .exec(&transaction)
                .await?;
        }

        transaction.commit().await?;

        self.find_by_id_with_relations(monastery.id)
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Failed to find a new created monastery".to_string(),
            ))
    }
}

/// Represents an element that handles all [`Monastery Monk`][`monastery_monks`] database topics.
#[derive(Default, Clone)]
pub struct MonasteryMonksRepository {}

impl MonasteryMonksRepository {
    /// Inserts a [`Monastery Monk`][`monastery_monks::Model`].
    pub async fn insert(
        &self,
        new_monastery_monk: monastery_monks::ActiveModel,
    ) -> Result<monastery_monks::Model, DbErr> {
        new_monastery_monk
            .insert(DatabaseClient::get_connection())
            .await
    }
}
