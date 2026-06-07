use entity::players;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use crate::{
    features::{actor::error::ActorErrorKind, player::error::PlayerErrorKind},
    shared::error::DomainError,
};

/// Represents an element that handles all [Player][`crate::features::player::domain::Player`] database topics.
#[derive(Default, Clone)]
pub struct PlayerRepository;

impl PlayerRepository {
    /// Finds a [`Player`][players::Model] for the provided ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<players::Model>, DomainError<PlayerErrorKind>> {
        players::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(PlayerErrorKind::FindById).with_cause(error))
    }

    /// Finds [`Players`][Vec<players::Model>] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<players::Model>, DomainError<PlayerErrorKind>> {
        players::Entity::find()
            .filter(players::Column::Id.is_in(ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(PlayerErrorKind::GetByIds).with_cause(error))
    }

    /// Finds the [`Players`][Vec<players::Model>] for the provided Process ID.
    pub async fn get_by_process_id<C: ConnectionTrait>(
        &self,
        process_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<players::Model>, DomainError<ActorErrorKind>> {
        players::Entity::find()
            .filter(players::Column::AssignedProcessId.eq(*process_id))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::GetByProcessId).with_cause(error))
    }

    /// Gets the [`Players`][Vec<players::Model>] for the provided Process IDs.
    pub async fn get_by_process_ids<C: ConnectionTrait>(
        &self,
        process_ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<players::Model>, DomainError<ActorErrorKind>> {
        players::Entity::find()
            .filter(players::Column::AssignedProcessId.is_in(process_ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::GetByProcessIds).with_cause(error))
    }

    /// Creates a [`Player`][players::Model].
    pub async fn create<C: ConnectionTrait>(
        &self,
        model: players::ActiveModel,
        db_connection: &C,
    ) -> Result<players::Model, DomainError<PlayerErrorKind>> {
        players::Entity::insert(model)
            .exec_with_returning(db_connection)
            .await
            .map_err(|error| DomainError::from(PlayerErrorKind::Creation).with_cause(error))
    }

    /// Updates a [`Player`][players::ActiveModel].
    pub async fn update<C: ConnectionTrait>(
        &self,
        model: players::ActiveModel,
        db_connection: &C,
    ) -> Result<players::Model, DomainError<PlayerErrorKind>> {
        players::Entity::update(model)
            .exec(db_connection)
            .await
            .map_err(|error| DomainError::from(PlayerErrorKind::Update).with_cause(error))
    }
}
