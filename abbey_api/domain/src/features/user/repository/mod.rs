use entity::{user_games, users};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use crate::{features::user::error::UserErrorKind, shared::error::DomainError};

/// Represents an element that handles all [`User`] database topics.
#[derive(Default, Clone)]
pub struct UserRepository;

impl UserRepository {
    /// Finds a [`User`][users::Model] by its ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<users::Model>, DomainError<UserErrorKind>> {
        users::Entity::find()
            .filter(users::Column::Id.eq(*id))
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(UserErrorKind::FindById).with_cause(error))
    }

    /// Gets the [`User Games`][user_games::Model] with the provided User ID.
    pub async fn get_user_games_by_user_id<C: ConnectionTrait>(
        &self,
        user_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<user_games::Model>, DomainError<UserErrorKind>> {
        user_games::Entity::find()
            .filter(user_games::Column::UserId.eq(*user_id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
            })
    }

    /// Finds a [`User`][users::Model] by its email.
    pub async fn find_by_email<C: ConnectionTrait>(
        &self,
        email: &String,
        connection: &C,
    ) -> Result<Option<users::Model>, DomainError<UserErrorKind>> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(connection)
            .await
            .map_err(|error| DomainError::from(UserErrorKind::FindByEmail).with_cause(error))
    }

    /// Creates a new [`User`] and persists it in the database.
    pub async fn create<C: ConnectionTrait>(
        &self,
        user: users::ActiveModel,
        db_connection: &C,
    ) -> Result<users::Model, DomainError<UserErrorKind>> {
        user.insert(db_connection)
            .await
            .map_err(|error| DomainError::from(UserErrorKind::Insert).with_cause(error))
    }

    /// Assign a Game to a [`User`][user_games::Model].
    pub async fn assign_game<C: ConnectionTrait>(
        &self,
        user_game_model: user_games::ActiveModel,
        db_connection: &C,
    ) -> Result<user_games::Model, DomainError<UserErrorKind>> {
        user_game_model
            .insert(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(
                    super::error::UserCreationErrorKind::AssignGame,
                ))
                .with_cause(error)
            })
    }
}
