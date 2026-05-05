use entity::{surroundings, surroundings_sources};
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use time::OffsetDateTime;

use crate::{
    features::{
        source::domain::Source,
        surroundings::{
            domain::Surroundings, error::SurroundingsErrorKind,
            forms::SurroundingSourceCreationForm,
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents an element that maps [`Surroundings`] elements.
pub struct SurroundingsMapper;

impl SurroundingsMapper {
    /// Maps a [`SurroundingsCreationForm`] to a [model][`surroundings::ActiveModel`] to create.
    pub fn to_new_active_model() -> surroundings::ActiveModel {
        surroundings::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
        }
    }

    /// Maps a [`Surroundings`] to a [model][`surroundings::ActiveModel`] to update.
    pub fn to_update_active_model(surroundings: Surroundings) -> surroundings::ActiveModel {
        surroundings::ActiveModel {
            id: Unchanged(surroundings.id().unwrap()),
            created_at: Unchanged(surroundings.created_at().unwrap()),
            last_updated_at: Set(Some(OffsetDateTime::now_utc())),
        }
    }

    /// Maps a [model][`surroundings::Model`] to a [`Surroundings`].
    pub fn to_domain_entity(
        model: surroundings::Model,
        sources: Vec<Source>,
    ) -> Result<Surroundings, DomainError<SurroundingsErrorKind>> {
        Surroundings::restore(model.id, model.created_at, model.last_updated_at, sources)
    }
}

/// Represents a mapper for [`SurroundingsSource`][surroundings_sources::Entity].
pub struct SurroundingsSourceMapper;

impl SurroundingsSourceMapper {
    /// Maps a [SurroundingSourceCreationForm] to a new [`model`][surroundings_sources::ActiveModel]
    pub fn to_new_active_model(
        creation_form: SurroundingSourceCreationForm,
    ) -> surroundings_sources::ActiveModel {
        surroundings_sources::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            surroundings_id: Set(creation_form.surroundings_id),
            source_id: Set(creation_form.source_id),
        }
    }
}
