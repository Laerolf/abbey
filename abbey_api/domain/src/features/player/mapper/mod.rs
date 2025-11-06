use entity::player;
use sea_orm::ActiveValue::{self, NotSet, Set};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::features::{
    player::{domain::Player, forms::PlayerCreationForm},
    process::domain::Process,
};

/// Represents an element that maps [`Player`] elements.
#[derive(Default)]
pub struct PlayerMapper;

impl PlayerMapper {
    /// Maps a [`PlayerCreationForm`] to a [model][`player::ActiveModel`] to create.
    pub fn to_new_active_model(&self, creation_form: PlayerCreationForm) -> player::ActiveModel {
        player::ActiveModel {
            id: NotSet,
            assigned_process_id: NotSet,
        }
    }

    /// Maps a [`Player`] to a [model][`player::ActiveModel`] to update.
    pub fn to_update_active_model(&self, player: &Player) -> player::ActiveModel {
        let assigned_process_id: Option<i32> = player
            .assigned_process
            .as_ref()
            .and_then(|weak_ref| weak_ref.upgrade())
            .map(|upgraded_ref| upgraded_ref.borrow().id());

        player::ActiveModel {
            id: Set(player.id),
            assigned_process_id: Set(assigned_process_id),
        }
    }

    /// Maps a [model][`player::Model`] to a [`Player`].
    pub fn to_domain_entity(
        &self,
        model: player::Model,
        assigned_process: Option<Weak<RefCell<dyn Process>>>,
    ) -> Player {
        Player::new(model.id, assigned_process)
    }
}
