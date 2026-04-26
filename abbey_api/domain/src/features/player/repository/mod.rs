use entity::{cyclic_process_resources, cyclic_processes, players, resources};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter,
};

use crate::{
    features::{
        output::mapper::ResourceMapper,
        player::{
            domain::Player, error::PlayerErrorKind, forms::PlayerCreationForm, mapper::PlayerMapper,
        },
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
            .filter(cyclic_process_resources::Column::CylicProcessId.eq(process_model.id))
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

    /// Finds a [`Player`] by its ID.
    pub async fn find_by_id<C: ConnectionTrait>(
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

        Ok(Some(PlayerMapper::to_domain_entity(player_model, None)))
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

    /// Creates a [`Player`] with all its relations.
    pub async fn create(
        &self,
        form: PlayerCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Player, DomainError<PlayerErrorKind>> {
        let new_player: players::Model = PlayerMapper::to_new_active_model(form)
            .insert(db_transaction)
            .await
            .map_err(|error| DomainError::from(PlayerErrorKind::Creation).with_cause(error))?;

        self.find_by_id_with_relations(&new_player.id, db_transaction)
            .await?
            .ok_or(DomainError::from(PlayerErrorKind::Creation))
    }

    /// Updates a [`Player`].
    pub async fn update(
        &self,
        player: Player,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Player, DomainError<PlayerErrorKind>> {
        players::Entity::update(PlayerMapper::to_update_active_model(player.clone()))
            .exec(db_transaction)
            .await
            .map_err(|error| DomainError::from(PlayerErrorKind::Update).with_cause(error))?;

        self.get_by_id_with_relations(&player.id().unwrap(), db_transaction)
            .await
    }
}
