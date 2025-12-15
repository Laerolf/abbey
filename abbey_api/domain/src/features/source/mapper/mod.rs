use entity::sources;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};

use crate::features::{
    process::{domain::CyclicProcess, mapper::CyclicProcessMapper},
    source::{domain::Source, forms::SourceCreationForm, repository::SourceWithRelations},
};

/// Represents an element that maps [`Source`] elements.
pub struct SourceMapper;

impl SourceMapper {
    /// Maps a [`SourceCreationForm`] to a [model][`sources::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: SourceCreationForm) -> sources::ActiveModel {
        sources::ActiveModel {
            id: NotSet,
            name: Set(creation_form.name),
            cyclic_process_id: Set(creation_form.process_id),
            last_claim_at: Set(None),
        }
    }

    /// Maps a [`Source`] to a [model][`sources::ActiveModel`] to update.
    pub fn to_update_active_model(source: Source) -> sources::ActiveModel {
        let assigned_process_id: i32 = source.process.as_ref().lock().unwrap().id;

        sources::ActiveModel {
            id: Unchanged(source.id),
            name: Unchanged(source.name),
            cyclic_process_id: Set(assigned_process_id),
            last_claim_at: Set(source.last_claim_at),
        }
    }

    /// Maps a [model][`SourceWithRelations`] to a [`Source`].
    pub fn to_domain_entity_with_relations(relations: SourceWithRelations) -> Source {
        let process = CyclicProcessMapper::to_domain_entity(relations.process);

        Source::new(relations.source.id, relations.source.name, process)
    }

    /// Maps a [model][`sources::Model`] to a [`Source`].
    pub fn to_domain_entity(model: sources::Model, process: CyclicProcess) -> Source {
        Source::new(model.id, model.name, process)
    }
}
