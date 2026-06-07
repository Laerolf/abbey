use std::str::FromStr;

use entity::users;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use time::OffsetDateTime;

use crate::{
    features::{
        game::domain::Game,
        user::{
            domain::{Status, User},
            forms::UserBlueprint,
        },
    },
    shared::DomainElement,
};

/// Represents an element that maps [`User`] elements.
pub struct UserMapper;

impl UserMapper {
    /// Maps a [`UserBlueprint`] to a [model][`users::ActiveModel`] to create.
    pub fn to_new_active_model(blueprint: UserBlueprint) -> users::ActiveModel {
        users::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            email: Set(blueprint.email),
            password: Set(blueprint.password),
            status: Set(Status::New.to_string()),
        }
    }

    /// Maps a [`User`] to a [model][`users::ActiveModel`] to update.
    pub fn to_update_active_model(user: User) -> users::ActiveModel {
        users::ActiveModel {
            id: Unchanged(user.id().unwrap()),
            created_at: Unchanged(user.created_at().unwrap()),
            last_updated_at: Set(Some(OffsetDateTime::now_utc())),
            email: Set(user.email().to_string()),
            password: Set(user.password().to_string()),
            status: Set(user.status().to_string()),
        }
    }

    /// Maps a [model][`users::Model`] to a [`User`].
    pub fn to_domain_entity(model: users::Model, games: Vec<Game>) -> User {
        User::restore(
            model.id,
            model.created_at,
            model.last_updated_at,
            model.email,
            model.password,
            Status::from_str(&model.status).expect("A user should have a valid status."),
            games,
        )
    }
}
