use sea_orm::{DatabaseTransaction, TransactionTrait};

use crate::{
    features::{
        game::{domain::Game, service::GameService},
        user::{
            domain::User,
            error::{UserCreationErrorKind, UserErrorKind},
            forms::UserCreationForm,
            mapper::UserMapper,
            repository::{UserRepository, UserWithRelations},
        },
    },
    shared::{db::DatabaseClient, error::DomainError},
};

/// Represents a service handling the [`User`] topic.
#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
    game_service: GameService,
}

impl UserService {
    pub fn new(game_service: GameService) -> Self {
        Self {
            repository: UserRepository::default(),
            game_service,
        }
    }

    /// Finds a [`User`] by its email.
    pub async fn find_by_email(
        &self,
        email: impl Into<String>,
    ) -> Result<Option<User>, UserErrorKind> {
        let Some(model) = self
            .repository
            .find_by_email(&email.into())
            .await
            .map_err(|_error| UserErrorKind::FindByEmail)?
        else {
            return Ok(None);
        };

        Ok(Some(UserMapper::to_domain_entity(model)))
    }

    /// Creates a new [`User`].
    pub async fn create_user(
        &self,
        creation_form: UserCreationForm,
    ) -> Result<UserWithRelations, DomainError<UserErrorKind>> {
        let transaction = DatabaseClient::get_connection()
            .begin()
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::Unknown))
                    .with_cause(error)
            })?;

        let mut related_user = self
            .repository
            .create_with_relations_in_transaction(creation_form, &transaction)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::Unknown))
                    .with_cause(error)
            })?;

        let new_game = self
            .game_service
            .create_game_in_transaction(&transaction)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::CreateGame))
                    .with_cause(error)
            })?;

        related_user = self
            .assign_game_in_transaction(
                UserMapper::to_domain_entity(related_user.user),
                new_game,
                &transaction,
            )
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::AssignGame))
                    .with_cause(error)
            })?;

        transaction.commit().await.map_err(|error| {
            DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::Unknown))
                .with_cause(error)
        })?;

        Ok(related_user)
    }

    /// Assigns a [Game] to a [`User`][UserWithRelations].
    pub async fn assign_game_in_transaction(
        &self,
        mut user: User,
        game: Game,
        transaction: &DatabaseTransaction,
    ) -> Result<UserWithRelations, DomainError<UserErrorKind>> {
        user.game = Some(game);

        let related_user = self
            .repository
            .update_with_relations_in_transaction(user, transaction)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(UserCreationErrorKind::AssignGame))
                    .with_cause(error)
            })?;

        Ok(related_user)
    }
}
