use entity::games;

use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use crate::{features::game::error::GameErrorKind, shared::error::DomainError};

/// Represents an element that handles all [Game][`crate::features::game::domain::Game`] database topics.
#[derive(Default, Clone)]
pub struct GameRepository;

impl GameRepository {
    /// Creates a new [Game][`games::Model`] and persists it in the database.
    pub async fn create<C: ConnectionTrait>(
        &self,
        model: games::ActiveModel,
        db_connection: &C,
    ) -> Result<games::Model, DomainError<GameErrorKind>> {
        model
            .insert(db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))
    }

    /// Finds a [`Game`][games::Model] by its ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<games::Model>, DomainError<GameErrorKind>> {
        games::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::FindById).with_cause(error))
    }

    /// Gets the [`Games`][Vec<games::Model>] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<games::Model>, DomainError<GameErrorKind>> {
        games::Entity::find()
            .filter(games::Column::Id.is_in(ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::GetById).with_cause(error))
    }
}
