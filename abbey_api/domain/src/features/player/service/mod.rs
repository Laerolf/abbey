use sea_orm::{ConnectionTrait, DatabaseTransaction};

use crate::{
    features::{
        player::{
            domain::Player, error::PlayerErrorKind, forms::PlayerCreationForm,
            mapper::PlayerMapper, repository::PlayerRepository,
        },
        process::{domain::ProcessKind, service::cyclic_process::CyclicProcessQueryService},
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Player`] topic.
#[derive(Clone)]
pub struct PlayerService {
    repository: PlayerRepository,
}

impl PlayerService {
    /// Creates a new [`PlayerService`].
    pub fn new(repository: PlayerRepository) -> Self {
        Self { repository }
    }

    /// Finds a [`Player`] by its ID and all its related entities.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<Player>, DomainError<PlayerErrorKind>> {
        self.repository
            .find_by_id_with_relations(id, db_connection)
            .await
    }

    /// Creates a new [`Player`].
    pub async fn create_player(
        &self,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Player, DomainError<PlayerErrorKind>> {
        let creation_form = PlayerCreationForm::new();

        self.repository.create(creation_form, db_transaction).await
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
}
