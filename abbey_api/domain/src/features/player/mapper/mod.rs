use entity::players;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use time::OffsetDateTime;

use crate::{
    features::{
        player::{domain::Player, forms::PlayerCreationForm},
        process::domain::ProcessKind,
    },
    shared::DomainElement,
};

/// Represents an element that maps [`Player`] elements.
pub struct PlayerMapper;

impl PlayerMapper {
    /// Maps a [`PlayerCreationForm`] to a [model][`players::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: PlayerCreationForm) -> players::ActiveModel {
        players::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            assigned_process_id: NotSet,
        }
    }

    /// Maps a [`Player`] to a [model][`players::ActiveModel`] to update.
    pub fn to_update_active_model(player: Player) -> players::ActiveModel {
        players::ActiveModel {
            id: Unchanged(player.id().unwrap()),
            created_at: Unchanged(player.created_at().unwrap()),
            last_updated_at: Set(Some(OffsetDateTime::now_utc())),
            assigned_process_id: Set(player
                .assigned_process()
                .as_ref()
                .map(|process| process.id().unwrap())),
        }
    }

    /// Maps a [model][`players::Model`] to a [`Player`].
    pub fn to_domain_entity(
        model: players::Model,
        assigned_process: Option<ProcessKind>,
    ) -> Player {
        Player::restore(
            model.id,
            model.created_at,
            model.last_updated_at,
            assigned_process,
        )
    }
}
