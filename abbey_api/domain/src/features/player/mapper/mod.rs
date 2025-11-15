use entity::players;
use sea_orm::ActiveValue::{NotSet, Set};
use std::{cell::RefCell, rc::Weak};

use crate::features::{
    player::{domain::Player, forms::PlayerCreationForm},
    process::domain::Process,
};

/// Represents an element that maps [`Player`] elements.
pub struct PlayerMapper;

impl PlayerMapper {
    /// Maps a [`PlayerCreationForm`] to a [model][`players::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: PlayerCreationForm) -> players::ActiveModel {
        players::ActiveModel {
            id: NotSet,
            assigned_process_id: NotSet,
        }
    }

    /// Maps a [`Player`] to a [model][`players::ActiveModel`] to update.
    pub fn to_update_active_model(player: &Player) -> players::ActiveModel {
        let assigned_process_id: Option<i32> = player
            .assigned_process
            .as_ref()
            .and_then(|weak_ref| weak_ref.upgrade())
            .map(|upgraded_ref| upgraded_ref.borrow().id());

        players::ActiveModel {
            id: Set(player.id),
            assigned_process_id: Set(assigned_process_id),
        }
    }

    /// Maps a [model][`players::Model`] to a [`Player`].
    pub fn to_domain_entity(
        model: players::Model,
        assigned_process: Option<Weak<RefCell<dyn Process>>>,
    ) -> Player {
        Player::new(model.id, assigned_process)
    }
}
