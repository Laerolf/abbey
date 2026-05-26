use entity::{
    cyclic_process_resources, cyclic_processes, monastery_monks, monk_skills, monks, resources,
    skills,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Statement,
};

use crate::{
    features::{
        actor::{
            domain::{ActorKind, monk::Monk},
            error::{ActorErrorKind, MonkErrorKind},
            forms::MonkCreationForm,
            mapper::MonkMapper,
        },
        output::mapper::ResourceMapper,
        process::{domain::ProcessKind, mapper::cyclic_process::CyclicProcessMapper},
        skill::{
            domain::Skill,
            forms::MonkSkillCreationForm,
            mapper::{MonkSkillMapper, SkillMapper},
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents an element that handles all [`Actor`][ActorKind] database topics.
#[derive(Default, Clone)]
pub struct ActorRepository;

impl ActorRepository {
    /// Finds an [`Actor`][ActorKind] for the provided ID and Game ID.
    pub async fn find_by_id_for_game<C: ConnectionTrait>(
        &self,
        id: &i32,
        game_id: &i32,
        db_connection: &C,
    ) -> Result<Option<ActorKind>, DomainError<ActorErrorKind>> {
        let Some(monk_model) = monks::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                db_connection.get_database_backend(),
                r#"
                        SELECT mo.*
                        FROM monks mo
                        INNER JOIN monastery_monks mm ON mm.monk_id = mo.id
                        INNER JOIN games g ON g.monastery_id = mm.monastery_id
                        WHERE g.id = $1
                        AND mo.id = $2
                    "#,
                [(*game_id).into(), (*id).into()],
            ))
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        let monk_skills: Vec<Skill> = skills::Entity::find()
            .inner_join(monk_skills::Entity)
            .filter(monk_skills::Column::MonkId.eq(monk_model.id))
            .distinct()
            .order_by_asc(skills::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect();

        let monk = ActorKind::Monk(
            MonkMapper::to_domain_entity(monk_model, monk_skills, None)
                .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?,
        );

        Ok(Some(monk))
    }

    /// Gets an [`Actor`][ActorKind] for the provided ID and Game ID.
    pub async fn get_by_id_for_game<C: ConnectionTrait>(
        &self,
        id: &i32,
        game_id: &i32,
        db_connection: &C,
    ) -> Result<ActorKind, DomainError<ActorErrorKind>> {
        self.find_by_id_for_game(id, game_id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(ActorErrorKind::GetById))
    }

    /// Finds [`Actors`][Vec<ActorKind>] for the provided IDs and Game ID.
    pub async fn find_many_by_ids_for_game<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        game_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<ActorKind>, DomainError<ActorErrorKind>> {
        let placeholders: Vec<String> = (2..=ids.len() + 1).map(|i| format!("${}", i)).collect();
        let in_clause = placeholders.join(", ");

        let sql = format!(
            r#"
                SELECT mo.*
                FROM monks mo
                INNER JOIN monastery_monks mm ON mm.monk_id = mo.id
                INNER JOIN monasteries mon ON mon.id = mm.monastery_id
                INNER JOIN games g ON g.monastery_id = mon.id
                WHERE g.id = $1
                AND mo.id IN ({})
            "#,
            in_clause
        );

        let mut values: Vec<sea_orm::Value> = vec![(*game_id).into()];
        values.extend(ids.iter().map(|id| (*id).into()));

        let all_monk_models = monks::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                db_connection.get_database_backend(),
                sql,
                values,
            ))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?;

        let all_monk_ids: Vec<i32> = all_monk_models
            .iter()
            .map(|monk_model| monk_model.id)
            .collect();

        let all_monk_skill_assignments = monk_skills::Entity::find()
            .filter(monk_skills::Column::MonkId.is_in(all_monk_ids.clone()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?;

        let all_monk_skills: Vec<Skill> = skills::Entity::find()
            .inner_join(monk_skills::Entity)
            .filter(monk_skills::Column::MonkId.is_in(all_monk_ids))
            .distinct()
            .order_by_asc(skills::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect();

        let all_monks: Vec<Monk> = all_monk_models
            .into_iter()
            .map(|monk_model| {
                let monk_skill_ids: Vec<i32> = all_monk_skill_assignments
                    .iter()
                    .filter(|skill_assignment_model| {
                        skill_assignment_model.monk_id == monk_model.id
                    })
                    .map(|skill_assignment_model| skill_assignment_model.skill_id)
                    .collect();

                let monk_skills = all_monk_skills
                    .iter()
                    .filter(|skill| monk_skill_ids.contains(&skill.id().unwrap()))
                    .cloned()
                    .collect();

                MonkMapper::to_domain_entity(monk_model, monk_skills, None)
                    .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))
            })
            .collect::<Result<Vec<Monk>, DomainError<ActorErrorKind>>>()?;

        let all_actors: Vec<ActorKind> = all_monks.into_iter().map(ActorKind::Monk).collect();

        Ok(all_actors)
    }
}

/// Represents an element that handles all [`Monk`] database topics.
#[derive(Default, Clone)]
pub struct MonkRepository;

impl MonkRepository {
    /// Finds a [`Monk`] by its ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<Monk>, DomainError<ActorErrorKind>> {
        let Some(monk_model) = monks::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        let monk_skills: Vec<Skill> = skills::Entity::find()
            .inner_join(monk_skills::Entity)
            .filter(monk_skills::Column::MonkId.eq(monk_model.id))
            .distinct()
            .order_by_asc(skills::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect();

        Ok(Some(MonkMapper::to_domain_entity(
            monk_model,
            monk_skills,
            None,
        )?))
    }

    /// Gets all [`Monk models`][Vec<monks::Model>] for the provided Monastery ID.
    pub async fn get_all_by_monastery_id<C: ConnectionTrait>(
        &self,
        monastery_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<monks::Model>, DomainError<ActorErrorKind>> {
        monks::Entity::find()
            .inner_join(monastery_monks::Entity)
            .filter(monastery_monks::Column::MonasteryId.eq(*monastery_id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ActorErrorKind::Monks(MonkErrorKind::GetAllByMonasteryId))
                    .with_cause(error)
            })
    }

    /// Gets all [`Monk skills`][Vec<monk_skills::Model>] for the provided Monk IDs.
    pub async fn get_skill_assignments_by_monk_ids<C: ConnectionTrait>(
        &self,
        monk_ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<monk_skills::Model>, DomainError<ActorErrorKind>> {
        monk_skills::Entity::find()
            .filter(monk_skills::Column::MonkId.is_in(monk_ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ActorErrorKind::GetSkillAssignmentsByIds).with_cause(error)
            })
    }

    /// Finds [`Monks`][Vec<monks::Model>] for the provided Process IDs.
    pub async fn find_by_process_ids<C: ConnectionTrait>(
        &self,
        process_ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<monks::Model>, DomainError<ActorErrorKind>> {
        monks::Entity::find()
            .filter(monks::Column::AssignedCyclicProcessId.is_in(process_ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindByProcessIds).with_cause(error))
    }

    /// Finds [`Monks`][Monk] by their ID.
    pub async fn find_many_by_ids_with_relations<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<Monk>, DomainError<ActorErrorKind>> {
        let all_monk_models = monks::Entity::find()
            .filter(monks::Column::Id.is_in(ids.to_owned()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::GetByIds).with_cause(error))?;

        let all_monk_skill_assignments = monk_skills::Entity::find()
            .filter(monk_skills::Column::MonkId.is_in(ids.to_owned()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::GetByIds).with_cause(error))?;

        let all_monk_skills: Vec<Skill> = skills::Entity::find()
            .inner_join(monk_skills::Entity)
            .filter(monk_skills::Column::MonkId.is_in(ids.to_owned()))
            .distinct()
            .order_by_asc(skills::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::GetByIds).with_cause(error))?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect();

        let monks = all_monk_models
            .into_iter()
            .map(|monk_model| {
                let monk_skill_ids: Vec<i32> = all_monk_skill_assignments
                    .iter()
                    .filter(|assignment_model| assignment_model.monk_id == monk_model.id)
                    .map(|assignment| assignment.skill_id)
                    .collect();

                let monk_skills: Vec<Skill> = all_monk_skills
                    .clone()
                    .into_iter()
                    .filter(|skill| monk_skill_ids.contains(&skill.id().unwrap()))
                    .collect();

                MonkMapper::to_domain_entity(monk_model, monk_skills, None).unwrap()
            })
            .collect();

        Ok(monks)
    }

    /// Finds the [assigned Process][`ProcessKind`] for the provided Monk ID.
    async fn find_assigned_process_with_relations<C: ConnectionTrait>(
        &self,
        monk_id: &i32,
        db_connection: &C,
    ) -> Result<Option<ProcessKind>, DomainError<ActorErrorKind>> {
        let Some(assigned_cyclic_process_model) = cyclic_processes::Entity::find()
            .filter(cyclic_processes::Column::Id.eq(*monk_id))
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        let assigned_cyclic_process_output_resources = resources::Entity::find()
            .inner_join(cyclic_process_resources::Entity)
            .filter(
                cyclic_process_resources::Column::CyclicProcessId
                    .eq(assigned_cyclic_process_model.id),
            )
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect();

        Ok(Some(ProcessKind::CyclicProcess(
            CyclicProcessMapper::to_domain_entity(
                assigned_cyclic_process_model,
                assigned_cyclic_process_output_resources,
                Vec::new(),
            )
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?,
        )))
    }

    /// Finds a [`Monk`] for the provided ID with its skills and assigned process.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<Monk>, DomainError<ActorErrorKind>> {
        let Some(monk_model) = monks::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        let monk_skills: Vec<Skill> = skills::Entity::find()
            .inner_join(monk_skills::Entity)
            .filter(monk_skills::Column::MonkId.eq(monk_model.id))
            .distinct()
            .order_by_asc(skills::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect();

        let assigned_process = self
            .find_assigned_process_with_relations(&monk_model.id, db_connection)
            .await?;

        Ok(Some(MonkMapper::to_domain_entity(
            monk_model,
            monk_skills,
            assigned_process,
        )?))
    }

    /// Gets a [`Monk`] for the provided ID with its skills and assigned process.
    pub async fn get_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Monk, DomainError<ActorErrorKind>> {
        self.find_by_id_with_relations(id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(ActorErrorKind::GetById))
    }

    /// Creates a new [`Monk`] and persists it in the database.
    pub async fn create(
        &self,
        form: MonkCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Monk, DomainError<ActorErrorKind>> {
        let monk_model: monks::Model = MonkMapper::to_new_active_model(form.clone())
            .insert(db_transaction)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Creation).with_cause(error))?;

        if !form.skill_ids.is_empty() {
            let skill_active_models: Vec<monk_skills::ActiveModel> = form
                .skill_ids
                .into_iter()
                .map(|skill_id| {
                    MonkSkillMapper::to_new_active_model(MonkSkillCreationForm::new(
                        monk_model.id,
                        skill_id,
                    ))
                })
                .collect();

            monk_skills::Entity::insert_many(skill_active_models)
                .exec(db_transaction)
                .await
                .map_err(|error| DomainError::from(ActorErrorKind::Creation).with_cause(error))?;
        }

        self.find_by_id_with_relations(&monk_model.id, db_transaction)
            .await?
            .ok_or(DomainError::from(ActorErrorKind::Creation))
    }

    /// Creates many [`Monk`][`Vec<Monk>`]s and persists them in the database.
    pub async fn create_many(
        &self,
        forms: Vec<MonkCreationForm>,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Vec<Monk>, DomainError<ActorErrorKind>> {
        let new_monk_active_models: Vec<monks::ActiveModel> = forms
            .clone()
            .into_iter()
            .map(MonkMapper::to_new_active_model)
            .collect();

        let monk_models = monks::Entity::insert_many(new_monk_active_models)
            .exec_with_returning_many(db_transaction)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Creation).with_cause(error))?;

        let new_active_skill_models: Vec<monk_skills::ActiveModel> = monk_models
            .iter()
            .zip(forms)
            .flat_map(|(monk, form)| {
                form.skill_ids.into_iter().map(|skill_id| {
                    MonkSkillMapper::to_new_active_model(MonkSkillCreationForm::new(
                        monk.id, skill_id,
                    ))
                })
            })
            .collect();

        if !new_active_skill_models.is_empty() {
            monk_skills::Entity::insert_many(new_active_skill_models)
                .exec(db_transaction)
                .await
                .map_err(|error| DomainError::from(ActorErrorKind::Creation).with_cause(error))?;
        }

        let monk_ids: Vec<i32> = monk_models.iter().map(|monk_model| monk_model.id).collect();

        self.find_many_by_ids_with_relations(&monk_ids, db_transaction)
            .await
    }

    pub async fn update(
        &self,
        monk: Monk,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Monk, DomainError<ActorErrorKind>> {
        monks::Entity::update(MonkMapper::to_update_active_model(monk.clone()))
            .exec(db_transaction)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Update).with_cause(error))?;

        self.get_by_id_with_relations(&monk.id()?, db_transaction)
            .await
    }
}
