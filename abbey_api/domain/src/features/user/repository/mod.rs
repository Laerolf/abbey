use entity::{games, users};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, ModelTrait, QueryFilter,
};

use crate::{
    features::user::{domain::User, forms::UserCreationForm, mapper::UserMapper},
    shared::db::DatabaseClient,
};

/// Represents an element that handles all [User][`super::domain::User`] database topics.
#[derive(Default, Clone)]
pub struct UserRepository {}

impl UserRepository {
    /// Finds a [`User`][`users::Model`] by its ID.
    pub async fn find_by_id(&self, id: &i32) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Column::Id.eq(*id))
            .one(DatabaseClient::get_connection())
            .await
    }

    /// Finds a [`User`][`users::Model`] by its ID.
    pub async fn find_by_id_in_transaction(
        &self,
        id: &i32,
        transaction: &DatabaseTransaction,
    ) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Column::Id.eq(*id))
            .one(transaction)
            .await
    }

    /// Finds a [`User`][`users::Model`] by its email.
    pub async fn find_by_email(&self, name: &String) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Column::Email.eq(name))
            .one(DatabaseClient::get_connection())
            .await
    }

    /// Finds a [`User`][`UserWithRelations`] by its ID and with all its related entities.
    pub async fn find_by_id_with_relations(
        &self,
        id: i32,
    ) -> Result<Option<UserWithRelations>, DbErr> {
        let Some(model) = self.find_by_id(&id).await? else {
            return Ok(None);
        };

        let db = DatabaseClient::get_connection();

        let game = model.find_related(games::Entity).one(db).await?;

        Ok(Some(UserWithRelations { user: model, game }))
    }

    /// Finds a [`User`][`UserWithRelations`] by its ID and with all its related entities.
    pub async fn find_by_id_with_relations_in_transaction(
        &self,
        id: i32,
        transaction: &DatabaseTransaction,
    ) -> Result<Option<UserWithRelations>, DbErr> {
        let Some(model) = self.find_by_id_in_transaction(&id, transaction).await? else {
            return Ok(None);
        };

        let game = model.find_related(games::Entity).one(transaction).await?;

        Ok(Some(UserWithRelations { user: model, game }))
    }

    /// Creates a [`User`][`users::Model`] with all its relations.
    pub async fn create_with_relations_in_transaction(
        &self,
        form: UserCreationForm,
        transaction: &DatabaseTransaction,
    ) -> Result<UserWithRelations, DbErr> {
        let user = UserMapper::to_new_active_model(form.clone())
            .insert(transaction)
            .await?;

        self.find_by_id_with_relations_in_transaction(user.id, transaction)
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Failed to the new created user".to_string(),
            ))
    }

    /// Updates a [`User`][`users::Model`] with all its relations.
    pub async fn update_with_relations_in_transaction(
        &self,
        user: User,
        transaction: &DatabaseTransaction,
    ) -> Result<UserWithRelations, DbErr> {
        let user = UserMapper::to_update_active_model(user)
            .update(transaction)
            .await?;

        self.find_by_id_with_relations_in_transaction(user.id, transaction)
            .await?
            .ok_or(DbErr::RecordNotFound("Failed to update a user".to_string()))
    }
}

/// A [`User`] with all its relations.
pub struct UserWithRelations {
    pub user: users::Model,
    pub game: Option<games::Model>,
}
