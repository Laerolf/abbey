use std::sync::Arc;

use axum::Router;
use domain::{
    features::{
        actor::{
            repository::{ActorRepository, MonkRepository},
            service::{ActorQueryService, MonkCommandService, MonkQueryService},
        },
        assignment::service::ProcessAssignmentService,
        auth::{
            repository::RefreshTokenRepository,
            service::{
                AuthenticationService, RefreshTokenCommandService, RefreshTokenQueryService,
                UserSessionQueryService,
            },
        },
        catalog::service::CatalogQueryService,
        game::{
            repository::GameRepository,
            service::{GameCommandService, GameQueryService},
        },
        monastery::{
            repository::MonasteryRepository,
            service::{MonasteryCommandService, MonasteryQueryService},
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
            repository::{cyclic_process::CyclicProcessRepository, task::TaskRepository},
            service::{
                cyclic_process::{
                    CyclicProcessCommandService, CyclicProcessQueryService, CyclicProcessService,
                },
                task::TaskService,
            },
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
        user::{
            repository::UserRepository,
            service::{UserCommandService, UserQueryService},
        },
    },
    shared::error::{DomainError, DomainErrorKind},
};
use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbErr, TransactionTrait};

/// The context of this API.
pub struct ApiContext<C: ConnectionTrait> {
    db_connection: Arc<C>,
    pub authentication_service: AuthenticationService,

    pub cyclic_process_service: CyclicProcessService,
    pub task_service: TaskService,
    pub process_assignment_service: ProcessAssignmentService,

    pub game_query_service: GameQueryService,
    pub game_command_service: GameCommandService,
    pub user_query_service: UserQueryService,
    pub catalog_query_service: CatalogQueryService,

    pub user_session_query_service: UserSessionQueryService,
}

impl<C: ConnectionTrait> ApiContext<C> {
    /// Creates a new [`ApiContext`].
    pub fn new(db_connection: Arc<C>) -> Self {
        let player_repository = PlayerRepository;
        let monk_repository = MonkRepository;
        let skill_repository = SkillRepository;
        let source_repository = SourceRepository;
        let cyclic_process_repository = CyclicProcessRepository;
        let task_repository = TaskRepository;
        let resource_repository = ResourceRepository;
        let monastery_repository = MonasteryRepository;
        let surroundings_repository = SurroundingsRepository;
        let user_repository = UserRepository;
        let game_repository = GameRepository;
        let refresh_token_repository = RefreshTokenRepository;
        let actor_repository = ActorRepository;

        let cyclic_process_service = CyclicProcessService::new(cyclic_process_repository.clone());
        let task_service = TaskService::new(task_repository.clone());

        let process_assignment_service = ProcessAssignmentService::new(
            player_repository.clone(),
            monk_repository.clone(),
            cyclic_process_repository.clone(),
            task_repository,
            actor_repository,
        );

        let refresh_token_query_service =
            RefreshTokenQueryService::new(refresh_token_repository.clone());

        let refresh_token_command_service = RefreshTokenCommandService::new(
            refresh_token_repository.clone(),
            refresh_token_query_service.clone(),
        );

        let catalog_query_service =
            CatalogQueryService::new(skill_repository.clone(), resource_repository.clone());

        let resource_query_service = ResourceQueryService::new(resource_repository.clone());
        let resource_command_service =
            ResourceCommandService::new(resource_repository, resource_query_service.clone());

        let skill_query_service = SkillQueryService::new(skill_repository.clone());
        let skill_command_service = SkillCommandService::new(skill_repository);

        let actor_query_service = ActorQueryService::new(
            player_repository.clone(),
            monk_repository.clone(),
            skill_query_service.clone(),
        );

        let cyclic_process_query_service = CyclicProcessQueryService::new(
            cyclic_process_repository.clone(),
            resource_query_service,
            actor_query_service,
        );
        let cyclic_process_command_service = CyclicProcessCommandService::new(
            cyclic_process_repository,
            cyclic_process_query_service.clone(),
        );

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

        let source_query_service = SourceQueryService::new(
            source_repository.clone(),
            cyclic_process_query_service.clone(),
        );
        let source_command_service =
            SourceCommandService::new(source_repository, source_query_service.clone());

        let surroundings_query_service =
            SurroundingsQueryService::new(surroundings_repository.clone(), source_query_service);
        let surroundings_command_service = SurroundingsCommandService::new(
            surroundings_repository,
            surroundings_query_service.clone(),
            source_command_service,
            resource_command_service,
            cyclic_process_command_service,
        );

        let monastery_query_service =
            MonasteryQueryService::new(monastery_repository.clone(), monk_query_service);
        let monastery_command_service = MonasteryCommandService::new(
            monastery_repository.clone(),
            monastery_query_service.clone(),
            monk_command_service,
        );

        let player_query_service =
            PlayerQueryService::new(player_repository.clone(), cyclic_process_query_service);
        let player_command_service =
            PlayerCommandService::new(player_repository, player_query_service.clone());

        let game_query_service = GameQueryService::new(
            game_repository.clone(),
            player_query_service,
            monastery_query_service,
            surroundings_query_service,
        );
        let game_command_service = GameCommandService::new(
            game_repository.clone(),
            game_query_service.clone(),
            monastery_command_service,
            player_command_service,
            surroundings_command_service,
        );

        let user_query_service =
            UserQueryService::new(user_repository.clone(), game_query_service.clone());
        let user_command_service = UserCommandService::new(
            user_repository.clone(),
            user_query_service.clone(),
            game_command_service.clone(),
        );

        let user_session_query_service = UserSessionQueryService::new(user_query_service.clone());

        let authentication_service = AuthenticationService::new(
            refresh_token_command_service,
            refresh_token_query_service,
            user_command_service,
            user_query_service.clone(),
        );

        Self {
            db_connection,
            authentication_service,

            cyclic_process_service,
            task_service,
            process_assignment_service,

            game_query_service,
            game_command_service,
            user_query_service,
            catalog_query_service,
            user_session_query_service,
        }
    }

    pub fn db_connection(&self) -> &C {
        &self.db_connection
    }
}

impl ApiContext<DatabaseConnection> {
    async fn begin_transaction(&self) -> Result<DatabaseTransaction, DbErr> {
        self.db_connection.begin().await
    }

    pub async fn in_transaction<F, T, E>(&self, f: F) -> Result<T, DomainError<E>>
    where
        F: AsyncFnOnce(&DatabaseTransaction) -> Result<T, DomainError<E>>,
        E: DomainErrorKind,
    {
        let db_transaction = self
            .begin_transaction()
            .await
            .map_err(|error| DomainError::from(E::unknown()).with_cause(error))?;

        let result = f(&db_transaction).await;

        match result {
            Ok(value) => {
                db_transaction
                    .commit()
                    .await
                    .map_err(|error| DomainError::from(E::unknown()).with_cause(error))?;
                Ok(value)
            }
            Err(error) => {
                db_transaction.rollback().await.ok();
                Err(error)
            }
        }
    }
}

/// Represents a API feature.
pub trait ApiFeature {
    /// Returns the [Routes][`Router<ApiContext>`] of this feature.
    fn routes() -> Router<ApiContext<DatabaseConnection>>;
}

impl<C: ConnectionTrait> Clone for ApiContext<C> {
    fn clone(&self) -> Self {
        Self {
            db_connection: self.db_connection.clone(),
            authentication_service: self.authentication_service.clone(),
            cyclic_process_service: self.cyclic_process_service.clone(),
            task_service: self.task_service.clone(),
            process_assignment_service: self.process_assignment_service.clone(),
            catalog_query_service: self.catalog_query_service.clone(),
            game_query_service: self.game_query_service.clone(),
            game_command_service: self.game_command_service.clone(),
            user_query_service: self.user_query_service.clone(),
            user_session_query_service: self.user_session_query_service.clone(),
        }
    }
}
