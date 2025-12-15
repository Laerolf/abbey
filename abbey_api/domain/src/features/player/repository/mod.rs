use entity::{cyclic_processes, players};
use sea_orm::{ActiveModelTrait, DatabaseTransaction, DbErr, EntityTrait, ModelTrait};

use crate::{
    features::player::{forms::PlayerCreationForm, mapper::PlayerMapper},
    shared::db::DatabaseClient,
};

/// [`Player`][`players::Model`] with all its related entities.
pub struct PlayerWithRelations {
    pub player: players::Model,
    pub cyclic_process: Option<cyclic_processes::Model>,
}

/// Represents an element that handles all [Player][`crate::features::player::domain::Player`] database topics.
#[derive(Default, Clone)]
pub struct PlayerRepository;

impl PlayerRepository {
    /// Finds a [Player][`players::Model`] by its ID.
    pub async fn find_by_id(&self, id: i32) -> Result<Option<players::Model>, DbErr> {
        players::Entity::find_by_id(id)
            .one(DatabaseClient::get_connection())
            .await
    }

    /// Finds a [Player][`players::Model`] by its ID.
    pub async fn find_by_id_in_transaction(
        &self,
        id: i32,
        transaction: &DatabaseTransaction,
    ) -> Result<Option<players::Model>, DbErr> {
        players::Entity::find_by_id(id).one(transaction).await
    }

    /// Finds a [Player][`players::Model`] by its ID and with all its related entities.
    pub async fn find_by_id_with_relations(
        &self,
        id: i32,
    ) -> Result<Option<PlayerWithRelations>, DbErr> {
        let Some(model) = self.find_by_id(id).await? else {
            return Ok(None);
        };

        let db = DatabaseClient::get_connection();

        let cyclic_process = model.find_related(cyclic_processes::Entity).one(db).await?;

        Ok(Some(PlayerWithRelations {
            player: model,
            cyclic_process,
        }))
    }

    /// Finds a [Player][`players::Model`] by its ID and with all its related entities.
    pub async fn find_by_id_with_relations_in_transaction(
        &self,
        id: i32,
        transaction: &DatabaseTransaction,
    ) -> Result<Option<PlayerWithRelations>, DbErr> {
        let Some(model) = self.find_by_id_in_transaction(id, transaction).await? else {
            return Ok(None);
        };

        let cyclic_process = model
            .find_related(cyclic_processes::Entity)
            .one(transaction)
            .await?;

        Ok(Some(PlayerWithRelations {
            player: model,
            cyclic_process,
        }))
    }

    /// Creates a [Player][`players::Model`] with all its relations.
    pub async fn create_with_relations_in_transaction(
        &self,
        form: PlayerCreationForm,
        transaction: &DatabaseTransaction,
    ) -> Result<PlayerWithRelations, DbErr> {
        let player = PlayerMapper::to_new_active_model(form)
            .insert(transaction)
            .await?;

        self.find_by_id_with_relations_in_transaction(player.id, transaction)
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Failed to find a new created player".to_string(),
            ))
    }
}
