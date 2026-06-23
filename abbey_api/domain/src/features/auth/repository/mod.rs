use crate::{
    features::auth::error::{AuthenticationErrorKind, RefreshErrorKind},
    shared::error::DomainError,
};
use entity::refresh_tokens;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

/// Represents an element that handles all [RefreshToken][`super::domain::RefreshToken`] database topics.
#[derive(Default, Clone)]
pub struct RefreshTokenRepository;

impl RefreshTokenRepository {
    /// Finds a [`RefreshToken`] with the provided ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<refresh_tokens::Model>, DomainError<AuthenticationErrorKind>> {
        refresh_tokens::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(RefreshErrorKind::FindById))
                    .with_cause(error)
            })
    }

    /// Finds a [`RefreshToken`][refresh_tokens::Model] with the provided User ID.
    pub async fn find_by_user_id<C: ConnectionTrait>(
        &self,
        user_id: &i32,
        db_connection: &C,
    ) -> Result<Option<refresh_tokens::Model>, DomainError<AuthenticationErrorKind>> {
        refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::UserId.eq(*user_id))
            .one(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::FindByUserId,
                ))
                .with_cause(error)
            })
    }

    /// Gets a [`RefreshToken`][refresh_tokens::Model] by its value, throws an error if it is not found.
    pub async fn find_by_value<C: ConnectionTrait>(
        &self,
        value: impl Into<&String>,
        db_connection: &C,
    ) -> Result<Option<refresh_tokens::Model>, DomainError<AuthenticationErrorKind>> {
        refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::Value.eq(value.into()))
            .one(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::RefreshTokenNotFound,
                ))
                .with_cause(error)
            })
    }

    /// Gets a [`RefreshToken`] with the provided ID.
    pub async fn get_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<refresh_tokens::Model, DomainError<AuthenticationErrorKind>> {
        self.find_by_id(id, db_connection)
            .await?
            .ok_or(DomainError::from(AuthenticationErrorKind::Refresh(
                RefreshErrorKind::GetById,
            )))
    }

    /// Deletes a [`RefreshToken`] by its ID.
    pub async fn delete_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<(), DomainError<AuthenticationErrorKind>> {
        let Some(refresh_token) = self.find_by_id(id, db_connection).await? else {
            return Ok(());
        };

        refresh_tokens::Entity::delete_by_id(refresh_token.id)
            .exec(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::DeleteRefreshToken,
                ))
                .with_cause(error)
            })?;

        Ok(())
    }

    /// Creates a new [`RefreshToken`][refresh_tokens::Model].
    pub async fn create<C: ConnectionTrait>(
        &self,
        new_refresh_token: refresh_tokens::ActiveModel,
        db_connection: &C,
    ) -> Result<refresh_tokens::Model, DomainError<AuthenticationErrorKind>> {
        new_refresh_token
            .insert(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::CreateRefreshToken,
                ))
                .with_cause(error)
            })
    }

    /// Deletes a [`RefreshToken`][refresh_tokens::Model].
    pub async fn delete<C: ConnectionTrait>(
        &self,
        refresh_token: refresh_tokens::ActiveModel,
        db_connection: &C,
    ) -> Result<(), DomainError<AuthenticationErrorKind>> {
        refresh_token.delete(db_connection).await.map_err(|error| {
            DomainError::from(AuthenticationErrorKind::Refresh(
                RefreshErrorKind::CreateRefreshToken,
            ))
            .with_cause(error)
        })?;

        Ok(())
    }
}
