use entity::sources;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};

use crate::features::{
    process::domain::{Process, cyclic_process::CyclicProcess},
    source::{domain::Source, forms::SourceCreationForm},
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
        sources::ActiveModel {
            id: Unchanged(source.id().unwrap()),
            name: Unchanged(source.name().to_string()),
            cyclic_process_id: Set(source.process().id().unwrap()),
            last_claim_at: Set(*source.last_claim_at()),
        }
    }

    /// Maps a [model][`sources::Model`] to a [`Source`].
    pub fn to_domain_entity(model: sources::Model, cyclic_process: CyclicProcess) -> Source {
        Source::from(model.id, model.name, cyclic_process)
    }
}
