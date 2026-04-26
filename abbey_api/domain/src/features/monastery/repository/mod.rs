use entity::{cyclic_processes, monasteries, monastery_monks, monk_skills, monks, skills};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter,
    QuerySelect,
};

use crate::{
    features::{
        actor::{
            domain::{Actor, monk::Monk},
            forms::MonasteryMonkCreationForm,
            mapper::{MonasteryMonkMapper, MonkMapper},
        },
        monastery::{
            domain::Monastery, error::MonasteryErrorKind, forms::MonasteryCreationForm,
            mapper::MonasteryMapper,
        },
        process::{domain::ProcessKind, mapper::cyclic_process::CyclicProcessMapper},
        skill::{domain::Skill, mapper::SkillMapper},
    },
    shared::error::DomainError,
};

/// Represents an element that handles all [Monastery][`super::domain::Monastery`] database topics.
#[derive(Default, Clone)]
pub struct MonasteryRepository;

impl MonasteryRepository {
    /// Gets the [Monks][monks::Model] of a [`Monastery`].
    async fn get_relations<C: ConnectionTrait>(
        &self,
        monastery_model: monasteries::Model,
        db_connection: &C,
    ) -> Result<Monastery, DomainError<MonasteryErrorKind>> {
        let monk_models = monks::Entity::find()
            .inner_join(monastery_monks::Entity)
            .filter(monastery_monks::Column::MonasteryId.eq(monastery_model.id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(MonasteryErrorKind::GetAllMonks).with_cause(error)
            })?;

        let monk_ids: Vec<i32> = monk_models.iter().map(|monk_model| monk_model.id).collect();

        let all_monk_skill_assignments = monk_skills::Entity::find()
            .filter(monk_skills::Column::MonkId.is_in(monk_ids.clone()))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(MonasteryErrorKind::GetAllMonks).with_cause(error)
            })?;

        let all_monk_skill_ids: Vec<i32> = all_monk_skill_assignments
            .iter()
            .map(|model| model.skill_id)
            .collect();

        let all_monk_skills: Vec<Skill> = skills::Entity::find()
            .filter(skills::Column::Id.is_in(all_monk_skill_ids.clone()))
            .distinct()
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::GetAllMonks).with_cause(error))?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect();

        let all_monk_processes = cyclic_processes::Entity::find()
            .filter(cyclic_processes::Column::Id.is_in(monk_ids))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(MonasteryErrorKind::GetAllMonks).with_cause(error)
            })?;

        let monks: Vec<Monk> = monk_models
            .into_iter()
            .map(|monk_model| {
                let monk_skill_ids: Vec<i32> = all_monk_skill_assignments
                    .iter()
                    .filter(|skill_assignment| skill_assignment.monk_id == monk_model.id)
                    .map(|skill_assignment| skill_assignment.skill_id)
                    .collect();

                let monk_skills: Vec<Skill> = all_monk_skills
                    .clone()
                    .into_iter()
                    .filter(|skill| monk_skill_ids.contains(&skill.id().unwrap()))
                    .collect();

                let assigned_process = monk_model
                    .assigned_cyclic_process_id
                    .and_then(|process_id| {
                        all_monk_processes
                            .iter()
                            .find(|p| p.id == process_id)
                            .cloned()
                    })
                    .map(|cyclic_process_model| {
                        ProcessKind::CyclicProcess(
                            CyclicProcessMapper::to_domain_entity(
                                cyclic_process_model,
                                Vec::new(),
                                Vec::new(),
                            )
                            .unwrap(),
                        )
                    });

                MonkMapper::to_domain_entity(monk_model, monk_skills, assigned_process).unwrap()
            })
            .collect();

        MonasteryMapper::to_domain_entity(monastery_model, monks)
            .map_err(|error| DomainError::from(MonasteryErrorKind::GetAllMonks).with_cause(error))
    }

    /// Finds a [`Monastery`] by its ID.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<Monastery>, DomainError<MonasteryErrorKind>> {
        let Some(monastery_model) = monasteries::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        Ok(Some(
            self.get_relations(monastery_model, db_connection).await?,
        ))
    }

    /// Creates a new [`Monastery`].
    pub async fn create(
        &self,
        form: MonasteryCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Monastery, DomainError<MonasteryErrorKind>> {
        let new_monastery: monasteries::Model = MonasteryMapper::to_new_active_model()
            .insert(db_transaction)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::Creation).with_cause(error))?;

        if !form.monks.is_empty() {
            let new_monastery_monks: Vec<monastery_monks::ActiveModel> = form
                .monks
                .iter()
                .map(|monk| {
                    MonasteryMonkMapper::to_new_active_model(MonasteryMonkCreationForm::new(
                        new_monastery.id,
                        monk.id().unwrap(),
                    ))
                })
                .collect();

            monastery_monks::Entity::insert_many(new_monastery_monks)
                .exec(db_transaction)
                .await
                .map_err(|error| {
                    DomainError::from(MonasteryErrorKind::Creation).with_cause(error)
                })?;
        }

        self.find_by_id_with_relations(&new_monastery.id, db_transaction)
            .await?
            .ok_or(DomainError::from(MonasteryErrorKind::Creation))
    }
}
