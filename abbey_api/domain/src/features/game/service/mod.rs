use sea_orm::ConnectionTrait;
use time::Duration;

use crate::{
    features::{
        game::{
            domain::{Game, game_engine::GameEngine},
            error::GameErrorKind,
            forms::GameBlueprint,
            mapper::GameMapper,
            repository::GameRepository,
        },
        monastery::{
            forms::MonasteryBlueprint,
            service::{MonasteryCommandService, MonasteryQueryService},
        },
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

        let monastery_blueprint =
            MonasteryBlueprint::generate(&monk_names, &skill_names, &mut engine)
                .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

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

#[cfg(test)]
pub mod game_command_service_tests {

    pub mod create {
        use sea_orm::{DatabaseBackend, MockDatabase};

        use crate::features::{
            actor::service::ActorQueryService,
            game::{
                repository::GameRepository,
                service::{GameCommandService, GameQueryService},
            },
            monastery::{
                repository::MonasteryRepository,
                service::{MonasteryCommandService, MonasteryQueryService},
            },
            monk::{
                repository::MonkRepository,
                service::{MonkCommandService, MonkQueryService},
            },
            output::{
                repository::ResourceRepository,
                service::{ResourceCommandService, ResourceQueryService},
            },
            player::{
                repository::PlayerRepository,
                service::{PlayerCommandService, PlayerQueryService},
            },
            process::{
                repository::cyclic_process::CyclicProcessRepository,
                service::cyclic_process::{CyclicProcessCommandService, CyclicProcessQueryService},
            },
            skill::{
                repository::SkillRepository,
                service::{SkillCommandService, SkillQueryService},
            },
            source::{
                repository::SourceRepository,
                service::{SourceCommandService, SourceQueryService},
            },
            surroundings::{
                repository::SurroundingsRepository,
                service::{SurroundingsCommandService, SurroundingsQueryService},
            },
        };

        #[tokio::test]
        #[ignore]
        pub async fn test_a_game_is_made_with_the_provided_game_seed() {
            // Given
            // TODO: Create repository traits + Create mocked repositories
            let db_connection = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

            let given_game_seed: u64 = 666666;

            let repository = GameRepository;
            let player_repository = PlayerRepository;
            let monk_repository = MonkRepository;
            let skill_repository = SkillRepository;
            let cyclic_process_repository = CyclicProcessRepository;
            let resource_repository = ResourceRepository;
            let monastery_repository = MonasteryRepository;
            let surroundings_repository = SurroundingsRepository;
            let source_repository = SourceRepository;

            let skill_query_service = SkillQueryService::new(skill_repository.clone());
            let skill_command_service = SkillCommandService::new(skill_repository);
            let actor_query_service = ActorQueryService::new(
                player_repository.clone(),
                monk_repository.clone(),
                skill_query_service.clone(),
            );
            let resource_query_service = ResourceQueryService::new(resource_repository.clone());
            let resource_command_service =
                ResourceCommandService::new(resource_repository, resource_query_service.clone());
            let cyclic_process_query_service = CyclicProcessQueryService::new(
                cyclic_process_repository.clone(),
                resource_query_service,
                actor_query_service,
            );
            let cyclic_process_command_service = CyclicProcessCommandService::new(
                cyclic_process_repository,
                cyclic_process_query_service.clone(),
            );
            let player_query_service = PlayerQueryService::new(
                player_repository.clone(),
                cyclic_process_query_service.clone(),
            );
            let player_command_service =
                PlayerCommandService::new(player_repository, player_query_service.clone());
            let monk_query_service = MonkQueryService::new(
                monk_repository.clone(),
                skill_query_service,
                cyclic_process_query_service.clone(),
            );
            let monk_command_service = MonkCommandService::new(
                monk_repository,
                monk_query_service.clone(),
                skill_command_service,
            );
            let source_query_service =
                SourceQueryService::new(source_repository.clone(), cyclic_process_query_service);
            let source_command_service =
                SourceCommandService::new(source_repository, source_query_service.clone());
            let monastery_query_service =
                MonasteryQueryService::new(monastery_repository.clone(), monk_query_service);
            let monastery_command_service = MonasteryCommandService::new(
                monastery_repository,
                monastery_query_service.clone(),
                monk_command_service,
            );
            let surroundings_query_service = SurroundingsQueryService::new(
                surroundings_repository.clone(),
                source_query_service,
            );
            let surroundings_command_service = SurroundingsCommandService::new(
                surroundings_repository,
                surroundings_query_service.clone(),
                source_command_service,
                resource_command_service,
                cyclic_process_command_service,
            );
            let game_query_service = GameQueryService::new(
                repository.clone(),
                player_query_service,
                monastery_query_service,
                surroundings_query_service,
            );

            let game_command_service = GameCommandService::new(
                repository,
                game_query_service,
                monastery_command_service,
                player_command_service,
                surroundings_command_service,
            );

            // When
            let game = game_command_service
                .create(given_game_seed, &db_connection)
                .await
                .expect("The Game can be created.");

            // Then
            let used_engine = game.game_engine();

            assert_eq!(
                given_game_seed.to_le_bytes().as_slice(),
                used_engine.generator().get_seed()
            );
        }
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
