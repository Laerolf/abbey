use std::{cell::RefCell, rc::Weak};

use entity::monks;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::features::{
    actor::{domain::monk::Monk, forms::MonkCreationForm},
    process::domain::Process,
    skill::domain::Skill,
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
    pub fn to_update_active_model(monk: &Monk) -> monks::ActiveModel {
        let assigned_cyclic_process_id: Option<i32> = monk
            .assigned_process
            .as_ref()
            .and_then(|weak_ref| weak_ref.upgrade())
            .map(|upgraded_ref| upgraded_ref.borrow().id());

        monks::ActiveModel {
            id: Set(monk.id),
            name: Set(monk.name.clone()),
            assigned_cyclic_process_id: Set(assigned_cyclic_process_id),
        }
    }

    /// Maps a [model][`monks::Model`] to a [`Monk`].
    pub fn to_domain_entity(
        model: monks::Model,
        skills: Vec<Skill>,
        assigned_process: Option<Weak<RefCell<dyn Process>>>,
    ) -> Monk {
        Monk::new(model.id, model.name, skills, assigned_process)
    }
}
