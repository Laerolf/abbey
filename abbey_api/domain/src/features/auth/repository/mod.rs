use crate::{
    features::auth::{
        domain::refresh_token::RefreshToken,
        error::{AuthenticationErrorKind, RefreshErrorKind},
        forms::RefreshTokenCreationForm,
        mapper::RefreshTokenMapper,
    },
    shared::error::DomainError,
};
use entity::refresh_tokens;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter,
};

/// Represents an element that handles all [RefreshToken][`super::domain::RefreshToken`] database topics.
#[derive(Default, Clone)]
pub struct RefreshTokenRepository;

impl RefreshTokenRepository {
    /// Finds a [`RefreshToken`] with the provided ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<RefreshToken>, DomainError<AuthenticationErrorKind>> {
        let Some(refresh_token_model) = refresh_tokens::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(RefreshErrorKind::FindById))
                    .with_cause(error)
            })?
        else {
            return Ok(None);
        };

        Ok(Some(RefreshTokenMapper::to_domain_entity(
            refresh_token_model,
        )))
    }

    /// Finds a [`RefreshToken`] with the provided User ID.
    pub async fn find_by_user_id<C: ConnectionTrait>(
        &self,
        user_id: &i32,
        db_connection: &C,
    ) -> Result<Option<RefreshToken>, DomainError<AuthenticationErrorKind>> {
        let Some(refresh_token_model) = refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::UserId.eq(*user_id))
            .one(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(RefreshErrorKind::FindById))
                    .with_cause(error)
            })?
        else {
            return Ok(None);
        };

        Ok(Some(RefreshTokenMapper::to_domain_entity(
            refresh_token_model,
        )))
    }

    /// Gets a [`RefreshToken`] by its value, throws an error if it is not found.
    pub async fn get_by_value<C: ConnectionTrait>(
        &self,
        value: impl Into<&String>,
        db_connection: &C,
    ) -> Result<RefreshToken, DomainError<AuthenticationErrorKind>> {
        let refresh_token_model = refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::Value.eq(value.into()))
            .one(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::RefreshTokenNotFound,
                ))
                .with_cause(error)
            })?
            .ok_or(DomainError::from(AuthenticationErrorKind::Refresh(
                RefreshErrorKind::RefreshTokenNotFound,
            )))?;

        Ok(RefreshTokenMapper::to_domain_entity(refresh_token_model))
    }

    /// Deletes a [`RefreshToken`] by its ID.
    pub async fn delete_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<(), DomainError<AuthenticationErrorKind>> {
        let Some(model) = self.find_by_id(id, db_connection).await? else {
            return Ok(());
        };

        refresh_tokens::Entity::delete_by_id(model.id().unwrap())
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

    /// Creates a new [`RefreshToken`].
    pub async fn create(
        &self,
        creation_form: RefreshTokenCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<RefreshToken, DomainError<AuthenticationErrorKind>> {
        if let Some(existing_user_refresh_token) = self
            .find_by_user_id(&creation_form.user_id, db_transaction)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(RefreshErrorKind::FindById))
                    .with_cause(error)
            })?
        {
            self.delete_by_id(&existing_user_refresh_token.id().unwrap(), db_transaction)
                .await?;
        }

        let new_refresh_token: refresh_tokens::Model =
            RefreshTokenMapper::to_new_active_model(creation_form)
                .insert(db_transaction)
                .await
                .map_err(|error| {
                    DomainError::from(AuthenticationErrorKind::Refresh(
                        RefreshErrorKind::CreateRefreshToken,
                    ))
                    .with_cause(error)
                })?;

        self.find_by_id(&new_refresh_token.id, db_transaction)
            .await?
            .ok_or(DomainError::from(AuthenticationErrorKind::Refresh(
                RefreshErrorKind::CreateRefreshToken,
            )))
    }
}
