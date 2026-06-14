use sea_orm::ConnectionTrait;
use time::Duration;

use crate::{
    features::{
        actor::error::ActorErrorKind,
        game::{
            domain::{Game, game_engine::GameEngine},
            error::GameErrorKind,
            forms::GameBlueprint,
            mapper::GameMapper,
            repository::GameRepository,
        },
        monastery::{
            error::MonasteryErrorKind,
            forms::MonasteryBlueprint,
            service::{MonasteryCommandService, MonasteryQueryService},
        },
        monk::forms::MonkBlueprint,
        output::{domain::resource::Category, forms::ResourceBlueprint},
        player::{
            forms::PlayerBlueprint,
            service::{PlayerCommandService, PlayerQueryService},
        },
        surroundings::{
            forms::{
                SurroundingsBlueprint, SurroundingsCyclicProcessSourceBlueprint,
                SurroundingsSourceBlueprint,
            },
            service::{SurroundingsCommandService, SurroundingsQueryService},
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// The default amount of Monks in a Monastery.
const DEFAULT_AMOUNT_OF_MONKS: i32 = 10;

/// Represents a command service for [`Games`][Game].
#[derive(Clone)]
pub struct GameCommandService {
    repository: GameRepository,
    game_query_service: GameQueryService,
    monastery_command_service: MonasteryCommandService,
    player_command_service: PlayerCommandService,
    surroundings_command_service: SurroundingsCommandService,
}

impl GameCommandService {
    /// Creates a new [`GameCommandService`].
    pub fn new(
        repository: GameRepository,
        game_query_service: GameQueryService,
        monastery_command_service: MonasteryCommandService,
        player_command_service: PlayerCommandService,
        surroundings_command_service: SurroundingsCommandService,
    ) -> Self {
        Self {
            repository,
            game_query_service,
            monastery_command_service,
            player_command_service,
            surroundings_command_service,
        }
    }

    /// Creates a new [Game].
    pub async fn create<C: ConnectionTrait>(
        &self,
        seed: u64,
        db_connection: &C,
    ) -> Result<Game, DomainError<GameErrorKind>> {
        let mut engine = GameEngine::new(seed);

        let skill_names = vec!["cooking".to_string(), "brewing".to_string()];
        let monk_names = vec!["Maurits".to_string()];

        let monk_blueprints = (0..DEFAULT_AMOUNT_OF_MONKS)
            .map(|_| {
                MonkBlueprint::new(
                    engine
                        .pick_random_element(&monk_names)
                        .ok_or_else(|| DomainError::from(ActorErrorKind::Creation))?,
                    skill_names.clone(),
                )
            })
            .collect::<Result<Vec<MonkBlueprint>, DomainError<ActorErrorKind>>>()
            .map_err(|error| DomainError::from(MonasteryErrorKind::MissingMonks).with_cause(error))
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        let monastery_blueprint = MonasteryBlueprint::new(monk_blueprints);

        let monastery = self
            .monastery_command_service
            .create(monastery_blueprint, db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        let player_blueprint = PlayerBlueprint::new();

        let player = self
            .player_command_service
            .create(player_blueprint, db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        let surroundings_blueprint =
            SurroundingsBlueprint::new(vec![SurroundingsSourceBlueprint::new(
                "the_beach",
                SurroundingsCyclicProcessSourceBlueprint::new(
                    vec![
                        ResourceBlueprint::new("sand", Category::Material),
                        ResourceBlueprint::new("seaweed", Category::Material),
                    ],
                    Duration::minutes(1),
                ),
            )]);

        let surroundings = self
            .surroundings_command_service
            .create(surroundings_blueprint, db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        let blueprint = GameBlueprint::new(
            engine
                .as_string()
                .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?,
            player.id().unwrap(),
            monastery.id().unwrap(),
            surroundings.id().unwrap(),
        );

        let new_model = self
            .repository
            .create(GameMapper::to_new_active_model(blueprint), db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        self.game_query_service
            .get_by_id(&new_model.id, db_connection)
            .await
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

    /// Gets a [`Game`] with the provided ID.
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

        let engine = GameEngine::from(&model.engine_state)?;

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
            engine,
            player,
            monastery,
            surroundings,
        ))
    }

    /// Gets the [`Games`][Vec<Game>] with the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<Game>, DomainError<GameErrorKind>> {
        let models = self.repository.get_by_ids(ids, db_connection).await?;

        let player_ids: Vec<i32> = models.iter().map(|model| model.player_id).collect();

        let players = self
            .player_query_service
            .get_by_ids(&player_ids, db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::PlayerNotFound).with_cause(error))?;

        let monastery_ids: Vec<i32> = models.iter().map(|model| model.monastery_id).collect();

        let monasteries = self
            .monastery_query_service
            .get_by_ids(&monastery_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error)
            })?;

        let surroundings_ids: Vec<i32> = models.iter().map(|model| model.surroundings_id).collect();

        let all_surroundings = self
            .surroundings_query_service
            .get_by_ids(&surroundings_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
            })?;

        models
            .into_iter()
            .map(|game_model| {
                let engine = GameEngine::from(&game_model.engine_state)?;

                let player = players
                    .iter()
                    .find(|model| {
                        model
                            .id()
                            .is_ok_and(|player_id| game_model.player_id == player_id)
                    })
                    .cloned()
                    .ok_or_else(|| DomainError::from(GameErrorKind::PlayerNotFound))?;

                let monastery = monasteries
                    .iter()
                    .find(|model| {
                        model
                            .id()
                            .is_ok_and(|monastery_id| game_model.monastery_id == monastery_id)
                    })
                    .cloned()
                    .ok_or_else(|| DomainError::from(GameErrorKind::MonasteryNotFound))?;

                let surroundings = all_surroundings
                    .iter()
                    .find(|model| {
                        model.id().is_ok_and(|surroundings_id| {
                            game_model.surroundings_id == surroundings_id
                        })
                    })
                    .cloned()
                    .ok_or_else(|| DomainError::from(GameErrorKind::SurroundingsNotFound))?;

                Ok(GameMapper::to_domain_entity(
                    game_model,
                    engine,
                    player,
                    monastery,
                    surroundings,
                ))
            })
            .collect::<Result<Vec<Game>, DomainError<GameErrorKind>>>()
    }
}
