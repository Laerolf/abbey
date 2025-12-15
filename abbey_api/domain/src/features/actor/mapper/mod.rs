use std::sync::{Arc, Mutex};

use entity::monks;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};

use crate::features::{
    actor::{domain::monk::Monk, forms::MonkCreationForm, repository::MonkWithRelations},
    process::{domain::Process, mapper::CyclicProcessMapper},
    skill::mapper::SkillMapper,
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
        let assigned_cyclic_process_id: Option<i32> = monk
            .assigned_process
            .as_ref()
            .map(|as_ref| as_ref.lock().unwrap().id());

        monks::ActiveModel {
            id: Unchanged(monk.id),
            name: Unchanged(monk.name),
            assigned_cyclic_process_id: Set(assigned_cyclic_process_id),
        }
    }

    /// Maps a [model][`monks::Model`] to a [`Monk`].
    pub fn to_domain_entity(model: monks::Model) -> Monk {
        Monk::new(model.id, model.name, Vec::new(), None)
    }

    /// Maps a [model][`monks::Model`] to a [`Monk`].
    pub fn to_domain_entity_with_relations(relations: MonkWithRelations) -> Monk {
        let skills = relations
            .skills
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect();

        let assigned_cyclic_process: Option<Arc<Mutex<dyn Process>>> =
            relations.cyclic_process.map(|process_model| {
                let domain_entity = CyclicProcessMapper::to_domain_entity(process_model);
                Arc::new(Mutex::new(domain_entity)) as _
            });

        Monk::new(
            relations.monk.id,
            relations.monk.name,
            skills,
            assigned_cyclic_process,
        )
    }
}
