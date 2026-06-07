use entity::{cyclic_process_resources, cyclic_processes, players, resources};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use crate::{
    features::{
        actor::error::ActorErrorKind,
        output::mapper::ResourceMapper,
        player::{domain::Player, error::PlayerErrorKind, mapper::PlayerMapper},
        process::mapper::cyclic_process::CyclicProcessMapper,
    },
    shared::error::DomainError,
};

/// Represents an element that handles all [Player][`crate::features::player::domain::Player`] database topics.
#[derive(Default, Clone)]
pub struct PlayerRepository;

impl PlayerRepository {
    /// Gets the [Player][players::Model] of a [`Player`].
    async fn get_relations<C: ConnectionTrait>(
        &self,
        player_model: players::Model,
        db_connection: &C,
    ) -> Result<Player, DomainError<PlayerErrorKind>> {
        let Some(process_model) = cyclic_processes::Entity::find()
            .filter(cyclic_processes::Column::Id.eq(player_model.assigned_process_id))
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(PlayerErrorKind::FindById).with_cause(error))?
        else {
            return Ok(PlayerMapper::to_domain_entity(player_model, None));
        };

        let process_output_resources = resources::Entity::find()
            .inner_join(cyclic_process_resources::Entity)
            .filter(cyclic_process_resources::Column::CyclicProcessId.eq(process_model.id))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(PlayerErrorKind::FindById).with_cause(error))?
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect();

        let assigned_process = CyclicProcessMapper::to_process_kind(
            process_model,
            process_output_resources,
            Vec::new(),
        )
        .map_err(|error| DomainError::from(PlayerErrorKind::FindById).with_cause(error))
        .unwrap();

        Ok(PlayerMapper::to_domain_entity(
            player_model,
            Some(assigned_process),
        ))
    }

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

    /// Finds a [`Player`] by its ID and with all its related assigned process.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<Player>, DomainError<PlayerErrorKind>> {
        let Some(player_model) = players::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(PlayerErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        Ok(Some(self.get_relations(player_model, db_connection).await?))
    }

    /// Gets a [`Player`] by its ID and with all its assigned process.
    pub async fn get_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Player, DomainError<PlayerErrorKind>> {
        self.find_by_id_with_relations(id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(PlayerErrorKind::GetById))
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
