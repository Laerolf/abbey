use entity::skills;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::shared::db::DatabasePool;

/// Represents an element that handles all [`Skill`][`super::domain::Skill`] database topics.
#[derive(Default)]
pub struct SkillRepository {}

impl SkillRepository {
    /// Inserts a [`Skill`][`skills`].
    pub async fn insert(&self, new_skill: skills::ActiveModel) -> Result<skills::Model, DbErr> {
        new_skill.insert(DatabasePool::instance()).await
    }

    /// Finds a [Skill][`skills::Model`] by its name.
    pub async fn find_by_name(&self, name: &String) -> Result<Option<skills::Model>, DbErr> {
        skills::Entity::find()
            .filter(skills::Column::Name.eq(name))
            .one(DatabasePool::instance())
            .await
    }
}
