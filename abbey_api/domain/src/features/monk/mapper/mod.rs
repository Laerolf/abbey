use entity::monks;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use time::OffsetDateTime;

use crate::{
    features::{
        actor::{domain::Actor, error::ActorErrorKind},
        monk::{domain::Monk, forms::MonkCreationForm},
        process::domain::ProcessKind,
        skill::domain::Skill,
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents an element that maps [`Monk`] elements.
pub struct MonkMapper;

impl MonkMapper {
    /// Maps a [`MonkCreationForm`] to a [model][`monks::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: MonkCreationForm) -> monks::ActiveModel {
        monks::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            name: Set(creation_form.name),
            assigned_cyclic_process_id: NotSet,
        }
    }

    /// Maps a [`Monk`] to a [model][`monks::ActiveModel`] to update.
    pub fn to_update_active_model(monk: Monk) -> monks::ActiveModel {
        monks::ActiveModel {
            id: Unchanged(monk.id().unwrap()),
            created_at: Unchanged(monk.created_at().unwrap()),
            last_updated_at: Set(Some(OffsetDateTime::now_utc())),
            name: Unchanged(monk.name().to_string()),
            assigned_cyclic_process_id: Set(monk
                .assigned_process()
                .clone()
                .map(|process| process.id().unwrap())),
        }
    }

    /// Maps a [model][`monks::Model`] to a [`Monk`].
    pub fn to_domain_entity(
        model: monks::Model,
        skills: Vec<Skill>,
        assigned_process: Option<ProcessKind>,
    ) -> Result<Monk, DomainError<ActorErrorKind>> {
        Monk::restore(
            model.id,
            model.created_at,
            model.last_updated_at,
            model.name,
            skills,
            assigned_process,
        )
    }
}
