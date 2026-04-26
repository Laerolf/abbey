use entity::{monastery_monks, monks};
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};

use crate::{
    features::{
        actor::{
            domain::{Actor, monk::Monk},
            error::ActorErrorKind,
            forms::{MonasteryMonkCreationForm, MonkCreationForm},
        },
        process::domain::{Process, ProcessKind},
        skill::domain::Skill,
    },
    shared::error::DomainError,
};

/// Represents an element that maps [`Monk`] elements.
pub struct MonkMapper;

impl MonkMapper {
    /// Maps a [`MonkCreationForm`] to a [model][`monks::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: MonkCreationForm) -> monks::ActiveModel {
        monks::ActiveModel {
            id: NotSet,
            name: Set(creation_form.name),
            assigned_cyclic_process_id: NotSet,
        }
    }

    /// Maps a [`Monk`] to a [model][`monks::ActiveModel`] to update.
    pub fn to_update_active_model(monk: Monk) -> monks::ActiveModel {
        monks::ActiveModel {
            id: Unchanged(monk.id().unwrap()),
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
        Monk::restore(model.id, model.name, skills, assigned_process)
    }
}

/// Represents a mapper for [`Monastery Monks`][monastery_monks::Entity].
pub struct MonasteryMonkMapper;

impl MonasteryMonkMapper {
    /// Maps a [MonasteryMonkCreationForm] to a [`model`][monastery_monks::ActiveModel].
    pub fn to_new_active_model(
        creation_form: MonasteryMonkCreationForm,
    ) -> monastery_monks::ActiveModel {
        monastery_monks::ActiveModel {
            id: NotSet,
            monastery_id: Set(creation_form.monastery_id),
            monk_id: Set(creation_form.monk_id),
        }
    }
}
