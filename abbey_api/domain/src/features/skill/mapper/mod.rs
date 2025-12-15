use entity::skills;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};

use crate::features::skill::{domain::Skill, forms::SkillCreationForm};

/// Represents an element that maps [`Skill`] elements.
pub struct SkillMapper;

impl SkillMapper {
    /// Maps a [`SkillCreationForm`] to a [model][`skills::ActiveModel`] to create.
    pub fn to_new_active_model(creation_form: SkillCreationForm) -> skills::ActiveModel {
        skills::ActiveModel {
            id: NotSet,
            name: Set(creation_form.name),
        }
    }

    /// Maps a [`Skill`] to a [model][`skills::ActiveModel`] to update.
    pub fn to_update_active_model(skill: Skill) -> skills::ActiveModel {
        skills::ActiveModel {
            id: Unchanged(skill.id),
            name: Unchanged(skill.name),
        }
    }

    /// Maps a [model][`skills::Model`] to a [`Skill`].
    pub fn to_domain_entity(model: skills::Model) -> Skill {
        Skill::new(model.id, model.name)
    }
}
