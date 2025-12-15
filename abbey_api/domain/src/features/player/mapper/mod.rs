use entity::players;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use std::sync::{Arc, Mutex};

use crate::features::{
    player::{domain::Player, forms::PlayerCreationForm, repository::PlayerWithRelations},
    process::{domain::Process, mapper::CyclicProcessMapper},
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
    pub fn to_update_active_model(player: Player) -> players::ActiveModel {
        let assigned_process_id: Option<i32> = player
            .assigned_process
            .as_ref()
            .map(|upgraded_ref| upgraded_ref.lock().unwrap().id());

        players::ActiveModel {
            id: Unchanged(player.id),
            assigned_process_id: Set(assigned_process_id),
        }
    }

    /// Maps a [model][`players::Model`] to a [`Player`].
    pub fn to_domain_entity(
        model: players::Model,
        assigned_process: Option<Arc<Mutex<dyn Process>>>,
    ) -> Player {
        Player::new(model.id, assigned_process)
    }

    /// Maps a [model][`PlayerWithRelations`] to a [`Player`].
    pub fn to_domain_entity_with_relations(relations: PlayerWithRelations) -> Player {
        let assigned_cyclic_process = relations.cyclic_process.map(|process_model| {
            let domain_entity = CyclicProcessMapper::to_domain_entity(process_model);
            Arc::new(Mutex::new(domain_entity)) as _
        });

        Player::new(relations.player.id, assigned_cyclic_process)
    }
}
