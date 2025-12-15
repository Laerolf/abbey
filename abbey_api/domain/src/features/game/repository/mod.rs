use entity::{games, monasteries, players, surroundings};
use sea_orm::{ActiveModelTrait, DatabaseTransaction, DbErr, EntityTrait, ModelTrait};

use crate::{
    features::game::{forms::GameCreationForm, mapper::GameMapper},
    shared::db::DatabaseClient,
};

/// Represents an element that handles all [Game][`crate::features::game::domain::Game`] database topics.
#[derive(Default, Clone)]
pub struct GameRepository;

impl GameRepository {
    /// Inserts a [Game][`games::Model`].
    pub async fn insert(&self, new_game: games::ActiveModel) -> Result<games::Model, DbErr> {
        new_game.insert(DatabaseClient::get_connection()).await
    }

    /// Finds a [Game][`games::Model`] by its ID.
    pub async fn find_by_id(&self, id: i32) -> Result<Option<games::Model>, DbErr> {
        games::Entity::find_by_id(id)
            .one(DatabaseClient::get_connection())
            .await
    }

    /// Finds a [Game][`games::Model`] by its ID.
    pub async fn find_by_id_in_transaction(
        &self,
        id: i32,
        transaction: &DatabaseTransaction,
    ) -> Result<Option<games::Model>, DbErr> {
        games::Entity::find_by_id(id).one(transaction).await
    }

    /// Finds a [Game][`games::Model`] by its ID and with all its related entities.
    pub async fn find_by_id_with_relations(
        &self,
        id: i32,
    ) -> Result<Option<GameWithRelations>, DbErr> {
        let Some(model) = self.find_by_id(id).await? else {
            return Ok(None);
        };

        let db = DatabaseClient::get_connection();

        let player = model
            .find_related(players::Entity)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::Custom("A game should have a player.".to_string()))?;

        let monastery = model
            .find_related(monasteries::Entity)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::Custom("A game should have a monastery.".to_string()))?;

        let surroundings = model
            .find_related(surroundings::Entity)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::Custom("A game should have surroundings.".to_string()))?;

        Ok(Some(GameWithRelations {
            game: model,
            player,
            monastery,
            surroundings,
        }))
    }

    /// Finds a [Game][`games::Model`] by its ID and with all its related entities.
    pub async fn find_by_id_with_relations_in_transaction(
        &self,
        id: i32,
        transaction: &DatabaseTransaction,
    ) -> Result<Option<GameWithRelations>, DbErr> {
        let Some(model) = self.find_by_id_in_transaction(id, transaction).await? else {
            return Ok(None);
        };

        let player = model
            .find_related(players::Entity)
            .one(transaction)
            .await?
            .ok_or_else(|| DbErr::Custom("A game should have a player.".to_string()))?;

        let monastery = model
            .find_related(monasteries::Entity)
            .one(transaction)
            .await?
            .ok_or_else(|| DbErr::Custom("A game should have a monastery.".to_string()))?;

        let surroundings = model
            .find_related(surroundings::Entity)
            .one(transaction)
            .await?
            .ok_or_else(|| DbErr::Custom("A game should have surroundings.".to_string()))?;

        Ok(Some(GameWithRelations {
            game: model,
            player,
            monastery,
            surroundings,
        }))
    }

    /// Creates a new [Game][`GameWithRelations`].
    pub async fn create_with_relations_in_transaction(
        &self,
        creation_form: GameCreationForm,
        transaction: &DatabaseTransaction,
    ) -> Result<GameWithRelations, DbErr> {
        let game = GameMapper::to_new_active_model(creation_form)
            .insert(transaction)
            .await?;

        self.find_by_id_with_relations_in_transaction(game.id, transaction)
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Failed to find a new created game".to_string(),
            ))
    }
}

/// [`Game`][`games::Model`] with all its related entities.
pub struct GameWithRelations {
    pub game: games::Model,
    pub player: players::Model,
    pub monastery: monasteries::Model,
    pub surroundings: surroundings::Model,
}
