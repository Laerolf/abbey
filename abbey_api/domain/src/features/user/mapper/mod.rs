use std::str::FromStr;

use entity::users;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};

use crate::features::user::{
    domain::{Status, User},
    forms::UserCreationForm,
    repository::UserWithRelations,
};

/// Represents an element that maps [`User`] elements.
pub struct UserMapper;

impl UserMapper {
    /// Maps a [`UserCreationForm`] to a [model][`users::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: UserCreationForm) -> users::ActiveModel {
        users::ActiveModel {
            id: NotSet,
            email: Set(creation_form.email),
            status: Set(Status::New.to_string()),
            game_id: NotSet,
        }
    }

    /// Maps a [`User`] to a [model][`users::ActiveModel`] to update.
    pub fn to_update_active_model(user: User) -> users::ActiveModel {
        users::ActiveModel {
            id: Unchanged(user.id),
            email: Set(user.email),
            status: Set(user.status.to_string()),
            game_id: Unchanged(user.game.as_ref().map(|game| game.id)),
        }
    }

    /// Maps a [model][`users::Model`] to a [`User`].
    pub fn to_domain_entity(model: users::Model) -> User {
        User::new(
            model.id,
            model.email,
            Status::from_str(&model.status).expect("A user should have a valid status."),
            None,
        )
    }

    /// Maps a [model][`UserWithRelations`] to a [`User`].
    pub fn to_domain_entity_with_relations(relations: UserWithRelations) -> User {
        // TODO: Return game with relations
        User::new(
            relations.user.id,
            relations.user.email,
            Status::from_str(&relations.user.status).expect("A user should have a status."),
            None,
        )
    }
}
