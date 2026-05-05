use std::sync::Arc;

use axum::Router;
use domain::{
    features::{
        actor::{
            repository::{ActorRepository, MonkRepository},
            service::MonkService,
        },
        assignment::service::ProcessAssignmentService,
        auth::{repository::RefreshTokenRepository, service::AuthenticationService},
        catalog::service::CatalogService,
        game::{repository::GameRepository, service::GameService},
        monastery::{repository::MonasteryRepository, service::MonasteryService},
        output::{repository::ResourceRepository, service::ResourceService},
        player::{repository::PlayerRepository, service::PlayerService},
        process::{
            repository::{cyclic_process::CyclicProcessRepository, task::TaskRepository},
            service::{cyclic_process::CyclicProcessService, task::TaskService},
        },
        skill::{repository::SkillRepository, service::SkillService},
        source::{repository::SourceRepository, service::SourceService},
        surroundings::{repository::SurroundingsRepository, service::SurroundingsService},
        user::{repository::UserRepository, service::UserService},
    },
    shared::error::{DomainError, DomainErrorKind},
};
use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbErr, TransactionTrait};

/// The context of this API.
// #[derive(Clone)]
pub struct ApiContext<C: ConnectionTrait> {
    db_connection: Arc<C>,
    pub authentication_service: AuthenticationService,
    pub user_service: UserService,
    pub game_service: GameService,
    pub cyclic_process_service: CyclicProcessService,
    pub task_service: TaskService,
    pub process_assignment_service: ProcessAssignmentService,
    pub catalog_service: CatalogService,
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

        let player_service = PlayerService::new(player_repository.clone());
        let monk_service = MonkService::new(monk_repository.clone());
        let skill_service = SkillService::new(skill_repository.clone());
        let source_service = SourceService::new(source_repository);
        let cyclic_process_service = CyclicProcessService::new(cyclic_process_repository.clone());
        let task_service = TaskService::new(task_repository.clone());
        let resource_service = ResourceService::new(resource_repository.clone());

        let process_assignment_service = ProcessAssignmentService::new(
            player_repository,
            monk_repository,
            cyclic_process_repository,
            task_repository,
            actor_repository,
        );
        let monastery_service =
            MonasteryService::new(monastery_repository, monk_service, skill_service);
        let surroundings_service = SurroundingsService::new(
            surroundings_repository,
            source_service,
            cyclic_process_service.clone(),
            resource_service,
        );
        let game_service = GameService::new(
            game_repository,
            monastery_service,
            player_service.clone(),
            surroundings_service,
        );
        let catalog_service = CatalogService::new(skill_repository, resource_repository);

        let user_service = UserService::new(user_repository, game_service.clone());
        let authentication_service =
            AuthenticationService::new(refresh_token_repository, user_service.clone());

        Self {
            db_connection,
            authentication_service,
            user_service,
            game_service,
            cyclic_process_service,
            task_service,
            process_assignment_service,
            catalog_service,
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
            user_service: self.user_service.clone(),
            game_service: self.game_service.clone(),
            cyclic_process_service: self.cyclic_process_service.clone(),
            task_service: self.task_service.clone(),
            process_assignment_service: self.process_assignment_service.clone(),
            catalog_service: self.catalog_service.clone(),
        }
    }
}
