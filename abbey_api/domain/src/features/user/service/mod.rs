use sea_orm::{ConnectionTrait, DatabaseTransaction};
use tracing::info;

use crate::{
    features::{
        game::{forms::UserGameCreationForm, service::GameService},
        user::{
            domain::User,
            error::{UserCreationErrorKind, UserErrorKind},
            forms::UserCreationForm,
            repository::UserRepository,
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a service handling the [`User`] topic.
#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
    game_service: GameService,
}

impl UserService {
    pub fn new(repository: UserRepository, game_service: GameService) -> Self {
        Self {
            repository,
            game_service,
        }
    }

    /// Finds a [`User`] by its ID.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<User>, DomainError<UserErrorKind>> {
        self.repository
            .find_by_id_with_relations(id, db_connection)
            .await
            .map_err(|error| DomainError::from(UserErrorKind::FindById).with_cause(error))
    }

    /// Get a [`User`] by its ID.
    pub async fn get_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<User, DomainError<UserErrorKind>> {
        self.repository
            .find_by_id_with_relations(id, db_connection)
            .await
            .map_err(|error| DomainError::from(UserErrorKind::FindById).with_cause(error))?
            .ok_or_else(|| DomainError::from(UserErrorKind::GetById))
    }

    /// Finds a [`User`] by its email.
    pub async fn find_by_email<C: ConnectionTrait>(
        &self,
        email: impl Into<&String>,
        db_connection: &C,
    ) -> Result<Option<User>, DomainError<UserErrorKind>> {
        self.repository
            .find_by_email_with_relations(email.into(), db_connection)
            .await
            .map_err(|error| DomainError::from(UserErrorKind::FindByEmail).with_cause(error))
    }

    /// Creates a new [`User`].
    pub async fn create_user(
        &self,
        creation_form: UserCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<User, DomainError<UserErrorKind>> {
        if (self
            .repository
            .find_by_email_with_relations(&creation_form.email, db_transaction)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::Unknown))
                    .with_cause(error)
            })?)
        .is_some()
        {
            return Err(DomainError::from(UserErrorKind::Creation(
                UserCreationErrorKind::EmailAlreadyExists(creation_form.email),
            )));
        }

        let mut new_user = self
            .repository
            .create(creation_form, db_transaction)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::Unknown))
                    .with_cause(error)
            })?;

        let new_game = self
            .game_service
            .create_game(db_transaction)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::CreateGame))
                    .with_cause(error)
            })?;

        new_user = self
            .assign_game(
                UserGameCreationForm::new(new_user.id()?, new_game.id().unwrap()),
                db_transaction,
            )
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::AssignGame))
                    .with_cause(error)
            })?;

        info!("Created a new user.");

        Ok(new_user)
    }

    /// Assigns a Game to a [`User`].
    pub async fn assign_game(
        &self,
        creation_form: UserGameCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<User, DomainError<UserErrorKind>> {
        self.repository
            .assign_game(creation_form, db_transaction)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::AssignGame))
                    .with_cause(error)
            })
    }
}
