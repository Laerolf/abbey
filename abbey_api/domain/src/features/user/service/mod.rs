use entity::users;
use sea_orm::ConnectionTrait;
use tracing::info;

use crate::{
    features::{
        game::{
            dto::GameOptionsForm,
            forms::UserGameAssignmentForm,
            mapper::UserGameMapper,
            service::{GameCommandService, GameQueryService},
        },
        user::{
            domain::User,
            error::{UserCreationErrorKind, UserErrorKind},
            forms::UserBlueprint,
            mapper::UserMapper,
            repository::UserRepository,
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a command service for [`Users`][User].
#[derive(Clone)]
pub struct UserCommandService {
    repository: UserRepository,
    user_query_service: UserQueryService,
    game_command_service: GameCommandService,
}

impl UserCommandService {
    /// Creates a new [`UserCommandService`].
    pub fn new(
        repository: UserRepository,
        user_query_service: UserQueryService,
        game_command_service: GameCommandService,
    ) -> Self {
        Self {
            repository,
            user_query_service,
            game_command_service,
        }
    }

    /// Assigns a Game to a [`User`].
    pub async fn assign_game<C: ConnectionTrait>(
        &self,
        form: UserGameAssignmentForm,
        db_connection: &C,
    ) -> Result<(), DomainError<UserErrorKind>> {
        self.repository
            .assign_game(UserGameMapper::to_new_active_model(form), db_connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::AssignGame))
                    .with_cause(error)
            })?;

        Ok(())
    }

    /// Creates a new [`User`].
    pub async fn create<C: ConnectionTrait>(
        &self,
        blueprint: UserBlueprint,
        game_options: GameOptionsForm,
        db_connection: &C,
    ) -> Result<User, DomainError<UserErrorKind>> {
        if self
            .user_query_service
            .exists_by_email(&blueprint.email, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::Unknown))
                    .with_cause(error)
            })?
        {
            return Err(DomainError::from(UserErrorKind::Creation(
                UserCreationErrorKind::EmailAlreadyExists(blueprint.email),
            )));
        }

        let new_model = self
            .repository
            .create(UserMapper::to_new_active_model(blueprint), db_connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::Unknown))
                    .with_cause(error)
            })?;

        // TODO: Get better seeds
        let game_seed: u64 = game_options.game_seed.unwrap_or(66666);

        let new_game = self
            .game_command_service
            .create(game_seed, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::CreateGame))
                    .with_cause(error)
            })?;

        self.assign_game(
            UserGameAssignmentForm::new(new_model.id, new_game.id().unwrap()),
            db_connection,
        )
        .await
        .map_err(|error| {
            DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::AssignGame))
                .with_cause(error)
        })?;

        info!("Created a new user.");

        self.user_query_service
            .get_by_id(&new_model.id, db_connection)
            .await
    }
}

/// Represents a query service for [`Users`][User].
#[derive(Clone)]
pub struct UserQueryService {
    repository: UserRepository,
    game_query_service: GameQueryService,
}

impl UserQueryService {
    /// Creates a new [`UserQueryService`].
    pub fn new(repository: UserRepository, game_query_service: GameQueryService) -> Self {
        Self {
            repository,
            game_query_service,
        }
    }

    /// Gets a [`User`] with the provided ID.
    pub async fn get_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<User, DomainError<UserErrorKind>> {
        let model = self
            .repository
            .find_by_id(id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(UserErrorKind::FindById))?;

        self.shape(model, db_connection).await
    }

    /// Gets a [`User`] with the provided ID.
    pub async fn get_by_email<C: ConnectionTrait>(
        &self,
        email: &String,
        db_connection: &C,
    ) -> Result<User, DomainError<UserErrorKind>> {
        let model = self
            .repository
            .find_by_email(email, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(UserErrorKind::FindById))?;

        self.shape(model, db_connection).await
    }

    /// Tests whether a [`User`] with the provided email exists.
    pub async fn exists_by_email<C: ConnectionTrait>(
        &self,
        email: &String,
        db_connection: &C,
    ) -> Result<bool, DomainError<UserErrorKind>> {
        Ok(self
            .repository
            .find_by_email(email, db_connection)
            .await?
            .is_some())
    }

    /// Puts a [`User`] together.
    async fn shape<C: ConnectionTrait>(
        &self,
        model: users::Model,
        db_connection: &C,
    ) -> Result<User, DomainError<UserErrorKind>> {
        let user_game_ids: Vec<i32> = self
            .repository
            .get_user_games_by_user_id(&model.id, db_connection)
            .await?
            .into_iter()
            .map(|model| model.game_id)
            .collect();

        let games = self
            .game_query_service
            .get_by_ids(&user_game_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
            })?;

        Ok(UserMapper::to_domain_entity(model, games))
    }
}
