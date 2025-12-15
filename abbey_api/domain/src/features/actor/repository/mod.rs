use std::collections::HashMap;

use entity::{cyclic_processes, monk_skills, monks, skills};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, ModelTrait, QueryFilter,
    TransactionTrait,
};

use crate::{
    features::actor::{forms::MonkCreationForm, mapper::MonkMapper},
    shared::db::DatabaseClient,
};

/// A [`Monk`][`monks::Model`] with all its related entities.
pub struct MonkWithRelations {
    pub monk: monks::Model,
    pub skills: Vec<skills::Model>,
    pub cyclic_process: Option<cyclic_processes::Model>,
}

/// Represents an element that handles all [`Monk`][`super::domain::monk`] database topics.
#[derive(Default, Clone)]
pub struct MonkRepository {}

impl MonkRepository {
    /// Finds a [`Monk`][`monks::Model`] by its ID.
    pub async fn find_by_id(&self, id: i32) -> Result<Option<monks::Model>, DbErr> {
        monks::Entity::find_by_id(id)
            .one(DatabaseClient::get_connection())
            .await
    }

    /// Finds [`Monks`][`Vec<monks::Model>`] by their ID.
    pub async fn find_many_by_id(&self, ids: Vec<i32>) -> Result<Vec<monks::Model>, DbErr> {
        monks::Entity::find()
            .filter(monks::Column::Id.is_in(ids))
            .all(DatabaseClient::get_connection())
            .await
    }

    /// Finds a [`Monk`][`MonkWithRelations`] with all its related entities, by its ID.
    pub async fn find_by_id_with_relations(
        &self,
        id: i32,
    ) -> Result<Option<MonkWithRelations>, DbErr> {
        let Some(monk_model) = self.find_by_id(id).await? else {
            return Ok(None);
        };

        let db = DatabaseClient::get_connection();

        let monk_skills = monk_model.find_related(monk_skills::Entity).all(db).await?;

        let skill_ids: Vec<i32> = monk_skills.iter().map(|skill| skill.id).collect();

        let skills = skills::Entity::find()
            .filter(skills::Column::Id.is_in(skill_ids))
            .all(db)
            .await?;

        let assigned_cyclic_process = monk_model
            .find_related(cyclic_processes::Entity)
            .one(db)
            .await?;

        Ok(Some(MonkWithRelations {
            monk: monk_model,
            skills,
            cyclic_process: assigned_cyclic_process,
        }))
    }

    /// Finds a [`Monks`][`Vec<MonkWithRelations>`] with all their related entities, by their ID.
    pub async fn find_many_by_id_with_relations(
        &self,
        ids: Vec<i32>,
    ) -> Result<Vec<MonkWithRelations>, DbErr> {
        let monk_models = self.find_many_by_id(ids).await?;

        let db = DatabaseClient::get_connection();

        let monk_ids: Vec<i32> = monk_models.iter().map(|monk| monk.id).collect();

        let monk_skill_models = monk_skills::Entity::find()
            .filter(monk_skills::Column::MonkId.is_in(monk_ids))
            .all(db)
            .await?;

        let skill_ids: Vec<i32> = monk_skill_models
            .iter()
            .map(|monk_skill| monk_skill.skill_id)
            .collect();

        let skill_models = skills::Entity::find()
            .filter(skills::Column::Id.is_in(skill_ids))
            .all(db)
            .await?;

        let process_ids: Vec<i32> = monk_models
            .iter()
            .filter_map(|monk| monk.assigned_cyclic_process_id)
            .collect();

        let process_models = cyclic_processes::Entity::find()
            .filter(cyclic_processes::Column::Id.is_in(process_ids))
            .all(db)
            .await?;

        let mut skills_by_id: HashMap<i32, skills::Model> = skill_models
            .into_iter()
            .map(|skill| (skill.id, skill))
            .collect();

        let mut processes_by_id: HashMap<i32, cyclic_processes::Model> = process_models
            .into_iter()
            .map(|process| (process.id, process))
            .collect();

        let mut skills_by_monk: HashMap<i32, Vec<i32>> = HashMap::new();
        for monk_skill in monk_skill_models {
            skills_by_monk
                .entry(monk_skill.monk_id)
                .or_default()
                .push(monk_skill.skill_id);
        }

        let related_monks: Vec<MonkWithRelations> = monk_models
            .into_iter()
            .map(|monk_model| {
                let skill_ids = skills_by_monk
                    .get(&monk_model.id)
                    .cloned()
                    .unwrap_or_default();

                let selected_skills: Vec<skills::Model> = skill_ids
                    .into_iter()
                    .filter_map(|id| skills_by_id.remove(&id))
                    .collect();

                let optional_process = monk_model
                    .assigned_cyclic_process_id
                    .and_then(|id| processes_by_id.remove(&id));

                MonkWithRelations {
                    monk: monk_model,
                    skills: selected_skills,
                    cyclic_process: optional_process,
                }
            })
            .collect();

        Ok(related_monks)
    }

    /// Creates a new [`Monk`][`MonkWithRelations`] with all its related entities.
    pub async fn create_with_relations(
        &self,
        form: MonkCreationForm,
    ) -> Result<MonkWithRelations, DbErr> {
        let db = DatabaseClient::get_connection();
        let transaction = db.begin().await?;

        let monk = MonkMapper::to_new_active_model(form.clone())
            .insert(&transaction)
            .await?;

        if !form.skill_ids.is_empty() {
            let monk_skills: Vec<monk_skills::ActiveModel> = form
                .skill_ids
                .iter()
                .map(|id| monk_skills::ActiveModel {
                    id: NotSet,
                    monk_id: Set(monk.id),
                    skill_id: Set(*id),
                })
                .collect();

            monk_skills::Entity::insert_many(monk_skills)
                .exec(&transaction)
                .await?;
        }

        transaction.commit().await?;

        self.find_by_id_with_relations(monk.id)
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Failed to find a new monk".to_string(),
            ))
    }

    /// Creates many [`Monks`][`Vec<MonkWithRelations>`] with all their related entities.
    pub async fn create_many_with_relations_in_transaction(
        &self,
        forms: Vec<MonkCreationForm>,
        transaction: &DatabaseTransaction,
    ) -> Result<Vec<MonkWithRelations>, DbErr> {
        let monk_forms: Vec<monks::ActiveModel> = forms
            .iter()
            .map(|form| MonkMapper::to_new_active_model(form.clone()))
            .collect();

        let monks = monks::Entity::insert_many(monk_forms)
            .exec_with_returning_many(transaction)
            .await?;

        let monk_skills: Vec<monk_skills::ActiveModel> = monks
            .iter()
            .zip(forms)
            .filter(|(_monk, form)| !form.skill_ids.is_empty())
            .flat_map(|(monk, form)| {
                form.skill_ids
                    .into_iter()
                    .map(|skill_id| monk_skills::ActiveModel {
                        id: NotSet,
                        monk_id: Set(monk.id),
                        skill_id: Set(skill_id),
                    })
            })
            .collect();

        if !monk_skills.is_empty() {
            monk_skills::Entity::insert_many(monk_skills)
                .exec(transaction)
                .await?;
        }

        self.find_many_by_id_with_relations(monks.iter().map(|monk| monk.id).collect())
            .await
    }
}
