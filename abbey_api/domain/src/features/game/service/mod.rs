use sea_orm::{ConnectionTrait, DatabaseTransaction};

use crate::{
    features::{
        game::{
            domain::Game, error::GameErrorKind, forms::GameCreationForm, mapper::GameMapper,
            repository::GameRepository,
        },
        monastery::service::{MonasteryQueryService, MonasteryService},
        player::service::{PlayerQueryService, PlayerService},
        surroundings::service::{SurroundingsQueryService, SurroundingsService},
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a service handling the [Game] topic.
#[derive(Clone)]
pub struct GameService {
    repository: GameRepository,
    monastery_service: MonasteryService,
    player_service: PlayerService,
    surroundings_service: SurroundingsService,
}

impl GameService {
    /// Creates a new [`GameService`].
    pub fn new(
        repository: GameRepository,
        monastery_service: MonasteryService,
        player_service: PlayerService,
        surroundings_service: SurroundingsService,
    ) -> Self {
        Self {
            repository,
            monastery_service,
            player_service,
            surroundings_service,
        }
    }

    /// Finds a [`Game`] by its ID.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<Game>, DomainError<GameErrorKind>> {
        let game = self
            .repository
            .find_by_id_with_relations(id, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(GameErrorKind::FindById)
                    .with_cause(error)
                    .with_context("ID", id.to_string())
            })?;

        Ok(game)
    }

    /// Gets a [`Game`] by its ID.
    pub async fn get_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Game, DomainError<GameErrorKind>> {
        self.find_by_id_with_relations(id, db_connection)
            .await?
            .ok_or(DomainError::from(GameErrorKind::GetById))
    }

    /// Creates a new [Game].
    pub async fn create_game(
        &self,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Game, DomainError<GameErrorKind>> {
        let monastery = self
            .monastery_service
            .create_monastery(db_transaction)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        let player = self
            .player_service
            .create_player(db_transaction)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        let surroundings = self
            .surroundings_service
            .create_surroundings(db_transaction)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        let creation_form = GameCreationForm::new(
            player.id().unwrap(),
            monastery.id().unwrap(),
            surroundings.id().unwrap(),
        );

        let related_game = self
            .repository
            .create(creation_form, db_transaction)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        Ok(related_game)
    }
}

/// Represents a query service for [`Games`][Game].
#[derive(Clone)]
pub struct GameQueryService {
    repository: GameRepository,
    player_query_service: PlayerQueryService,
    monastery_query_service: MonasteryQueryService,
    surroundings_query_service: SurroundingsQueryService,
}

impl GameQueryService {
    /// Creates a new [`GameQueryService`].
    pub fn new(
        repository: GameRepository,
        player_query_service: PlayerQueryService,
        monastery_query_service: MonasteryQueryService,
        surroundings_query_service: SurroundingsQueryService,
    ) -> Self {
        Self {
            repository,
            player_query_service,
            monastery_query_service,
            surroundings_query_service,
        }
    }

    /// Gets a [`Game`] for the provided ID.
    pub async fn get_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Game, DomainError<GameErrorKind>> {
        let model = self
            .repository
            .find_by_id(id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(GameErrorKind::GetById))?;

        let player = self
            .player_query_service
            .get_by_id(&model.player_id, db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::PlayerNotFound).with_cause(error))?;

        let monastery = self
            .monastery_query_service
            .get_by_id(&model.monastery_id, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error)
            })?;

        let surroundings = self
            .surroundings_query_service
            .get_by_id(&model.surroundings_id, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
            })?;

        Ok(GameMapper::to_domain_entity(
            model,
            player,
            monastery,
            surroundings,
        ))
    }
}
