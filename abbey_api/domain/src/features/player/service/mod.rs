use sea_orm::ConnectionTrait;

use crate::{
    features::{
        actor::error::ActorErrorKind,
        player::{
            domain::Player, error::PlayerErrorKind, forms::PlayerBlueprint, mapper::PlayerMapper,
            repository::PlayerRepository,
        },
        process::{domain::ProcessKind, service::cyclic_process::CyclicProcessQueryService},
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a command service for [`Players`][Player].
#[derive(Clone)]
pub struct PlayerCommandService {
    repository: PlayerRepository,
    player_query_service: PlayerQueryService,
}

impl PlayerCommandService {
    /// Creates a new [`PlayerCommandService`].
    pub fn new(repository: PlayerRepository, player_query_service: PlayerQueryService) -> Self {
        Self {
            repository,
            player_query_service,
        }
    }

    /// Creates a new [`Player`].
    pub async fn create<C: ConnectionTrait>(
        &self,
        blueprint: PlayerBlueprint,
        db_connection: &C,
    ) -> Result<Player, DomainError<PlayerErrorKind>> {
        let active_model = PlayerMapper::to_new_active_model(blueprint);

        let model = self.repository.create(active_model, db_connection).await?;

        self.player_query_service
            .get_by_id(&model.id, db_connection)
            .await
    }

    /// Updates a [`Player`].
    pub async fn update<C: ConnectionTrait>(
        &self,
        player: Player,
        db_connection: &C,
    ) -> Result<Player, DomainError<ActorErrorKind>> {
        let active_model = PlayerMapper::to_update_active_model(player);

        let updated_model = self
            .repository
            .update(active_model, db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Update).with_cause(error))?;

        self.player_query_service
            .get_by_id(&updated_model.id, db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Update).with_cause(error))
    }
}

/// Represents a query service for [`Players`][Player].
#[derive(Clone)]
pub struct PlayerQueryService {
    repository: PlayerRepository,
    cyclic_process_query_service: CyclicProcessQueryService,
}

impl PlayerQueryService {
    /// Creates a new [`PlayerQueryService`].
    pub fn new(
        repository: PlayerRepository,
        cyclic_process_query_service: CyclicProcessQueryService,
    ) -> Self {
        Self {
            repository,
            cyclic_process_query_service,
        }
    }

    /// Gets a [`Player`] with the provided ID.
    pub async fn get_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Player, DomainError<PlayerErrorKind>> {
        let model = self
            .repository
            .find_by_id(id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(PlayerErrorKind::FindById))?;

        let process = if let Some(process_id) = model.assigned_process_id {
            let cyclic_process = self
                .cyclic_process_query_service
                .get_by_id(&process_id, db_connection)
                .await
                .map_err(|error| DomainError::from(PlayerErrorKind::FindById).with_cause(error))?;

            Some(ProcessKind::CyclicProcess(cyclic_process))
        } else {
            None
        };

        Ok(PlayerMapper::to_domain_entity(model, process))
    }

    /// Gets the [`Player`][Vec<Player>] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<Player>, DomainError<PlayerErrorKind>> {
        let models = self.repository.get_by_ids(ids, db_connection).await?;

        let process_ids: Vec<i32> = models
            .iter()
            .filter_map(|player| player.assigned_process_id)
            .collect();

        let processes = self
            .cyclic_process_query_service
            .get_by_ids(&process_ids, db_connection)
            .await
            .map_err(|error| DomainError::from(PlayerErrorKind::GetByIds).with_cause(error))?;

        let players = models
            .into_iter()
            .map(|player_model| {
                let assigned_process: Option<ProcessKind> = processes
                    .iter()
                    .find(|process| process.id().ok() == player_model.assigned_process_id)
                    .cloned()
                    .map(ProcessKind::CyclicProcess);

                PlayerMapper::to_domain_entity(player_model, assigned_process)
            })
            .collect();

        Ok(players)
    }
}
