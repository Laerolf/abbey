use entity::sources;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use time::OffsetDateTime;

use crate::{
    features::{
        process::domain::cyclic_process::CyclicProcess,
        source::{domain::Source, forms::SourceCreationForm},
    },
    shared::DomainElement,
};

/// Represents an element that maps [`Source`] elements.
pub struct SourceMapper;

impl SourceMapper {
    /// Maps a [`SourceCreationForm`] to a [model][`sources::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: SourceCreationForm) -> sources::ActiveModel {
        sources::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            name: Set(creation_form.name),
            cyclic_process_id: Set(creation_form.process_id),
            last_claim_at: Set(None),
        }
    }

    /// Maps a [`Source`] to a [model][`sources::ActiveModel`] to update.
    pub fn to_update_active_model(source: Source) -> sources::ActiveModel {
        sources::ActiveModel {
            id: Unchanged(source.id().unwrap()),
            created_at: Unchanged(source.created_at().unwrap()),
            last_updated_at: Set(Some(OffsetDateTime::now_utc())),
            name: Unchanged(source.name().to_string()),
            cyclic_process_id: Set(source.process().id().unwrap()),
            last_claim_at: Set(*source.last_claim_at()),
        }
    }

    /// Maps a [model][`sources::Model`] to a [`Source`].
    pub fn to_domain_entity(model: sources::Model, cyclic_process: CyclicProcess) -> Source {
        Source::from(
            model.id,
            model.created_at,
            model.last_updated_at,
            model.name,
            cyclic_process,
        )
    }
}
