use entity::{monk_skills, skills};
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use time::OffsetDateTime;

use crate::{
    features::skill::{
        domain::Skill,
        forms::{MonkSkillAssignmentForm, SkillCreationForm},
    },
    shared::DomainElement,
};

/// Represents an mapper for [`Skills`][entity::skills::Entity].
pub struct SkillMapper;

impl SkillMapper {
    /// Maps a [`SkillCreationForm`] to a [model][`skills::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: SkillCreationForm) -> skills::ActiveModel {
        skills::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            name: Set(creation_form.name),
        }
    }

    /// Maps a [`Skill`] to a [model][`skills::ActiveModel`] to update.
    pub fn to_update_active_model(skill: Skill) -> skills::ActiveModel {
        skills::ActiveModel {
            id: Unchanged(skill.id().unwrap()),
            created_at: Unchanged(skill.created_at().unwrap()),
            last_updated_at: Set(Some(OffsetDateTime::now_utc())),
            name: Unchanged(skill.name().to_string()),
        }
    }

    /// Maps a [model][`skills::Model`] to a [`Skill`].
    pub fn to_domain_entity(model: skills::Model) -> Skill {
        Skill::restore(
            model.id,
            model.created_at,
            model.last_updated_at,
            model.name,
        )
    }
}

/// Represents a mapper for [`Monk Skills`][entity::monk_skills::Entity].
pub struct MonkSkillMapper;

impl MonkSkillMapper {
    /// Maps a [MonkSkillCreationForm] to a [`model`][monk_skills::ActiveModel].
    pub fn to_new_active_model(creation_form: MonkSkillAssignmentForm) -> monk_skills::ActiveModel {
        monk_skills::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            monk_id: Set(creation_form.monk_id),
            skill_id: Set(creation_form.skill_id),
        }
    }
}
