use std::collections::HashSet;

use entity::skills;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, QueryFilter,
};

use crate::{
    features::skill::{forms::SkillCreationForm, mapper::SkillMapper},
    shared::db::DatabaseClient,
};

/// Represents an element that handles all [`Skill`][`super::domain::Skill`] database topics.
#[derive(Default, Clone)]
pub struct SkillRepository {}

impl SkillRepository {
    /// Finds a [`Skill`][`skills::Model`] by its name.
    pub async fn find_by_name(&self, name: &String) -> Result<Option<skills::Model>, DbErr> {
        skills::Entity::find()
            .filter(skills::Column::Name.eq(name))
            .one(DatabaseClient::get_connection())
            .await
    }

    /// Finds a [`Skills`][`Vec<skills::Model>`] by their names.
    pub async fn find_many_by_name(
        &self,
        names: &Vec<String>,
    ) -> Result<Vec<skills::Model>, DbErr> {
        skills::Entity::find()
            .filter(skills::Column::Name.is_in(names))
            .all(DatabaseClient::get_connection())
            .await
    }

    /// Creates a [`Skill`][`skills::Model`].
    pub async fn create(&self, form: SkillCreationForm) -> Result<skills::Model, DbErr> {
        SkillMapper::to_new_active_model(form)
            .insert(DatabaseClient::get_connection())
            .await
    }

    /// Creates many [`Skills`][`Vec<skills::Model>`].
    pub async fn create_many_in_transaction(
        &self,
        forms: Vec<SkillCreationForm>,
        transaction: &DatabaseTransaction,
    ) -> Result<Vec<skills::Model>, DbErr> {
        let skill_models: Vec<skills::ActiveModel> = forms
            .into_iter()
            .map(|form| skills::ActiveModel {
                id: NotSet,
                name: Set(form.name),
            })
            .collect();

        skills::Entity::insert_many(skill_models)
            .exec_with_returning_many(transaction)
            .await
    }

    /// Finds a [`Skill`][`skills::Model`] by its name or creates it if it doesn't exist.
    pub async fn find_by_name_or_create(
        &self,
        form: SkillCreationForm,
    ) -> Result<skills::Model, DbErr> {
        match self.find_by_name(&form.name).await? {
            Some(model) => Ok(model),
            None => self.create(form).await,
        }
    }

    /// Finds many [`Skills`][`Vec<skills::Model>`] by their name or creates them if they don't exist.
    pub async fn find_by_name_or_create_many_in_transaction(
        &self,
        forms: Vec<SkillCreationForm>,
        transaction: &DatabaseTransaction,
    ) -> Result<Vec<skills::Model>, DbErr> {
        let skill_names = forms.iter().map(|form| form.name.clone()).collect();

        let found_skill_models = self.find_many_by_name(&skill_names).await?;

        let found_skill_names: HashSet<String> = found_skill_models
            .iter()
            .map(|skill| skill.name.clone())
            .collect();

        let missing_skill_forms: Vec<SkillCreationForm> = forms
            .into_iter()
            .filter(|form| !found_skill_names.contains(&form.name))
            .collect();

        let created_skill_models = if !missing_skill_forms.is_empty() {
            self.create_many_in_transaction(missing_skill_forms, transaction)
                .await?
        } else {
            Vec::new()
        };

        let mut all_skills = found_skill_models;
        all_skills.extend(created_skill_models);
        Ok(all_skills)
    }
}
