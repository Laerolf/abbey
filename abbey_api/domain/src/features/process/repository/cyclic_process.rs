use entity::{
    cyclic_process_resources, cyclic_processes, games, monk_skills, monks, players, resources,
    skills,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter,
    QuerySelect, Statement,
};

use crate::{
    features::{
        actor::{domain::ActorKind, mapper::MonkMapper},
        output::{domain::resource::Resource, mapper::ResourceMapper},
        player::mapper::PlayerMapper,
        process::{
            domain::{Process, cyclic_process::CyclicProcess},
            error::ProcessErrorKind,
            forms::cyclic_process::{CyclicProcessCreationForm, CyclicProcessResourceCreationForm},
            mapper::cyclic_process::{CyclicProcessMapper, CyclicProcessResourceMapper},
        },
        skill::{domain::Skill, mapper::SkillMapper},
    },
    shared::error::DomainError,
};

/// Represents an element that handles all [CyclicProcess] database topics.
#[derive(Default, Clone)]
pub struct CyclicProcessRepository;

impl CyclicProcessRepository {
    /// Gets all the related entities of a [`CyclicProcess`] for a Game.
    async fn get_relations_for_game<C: ConnectionTrait>(
        &self,
        process_model: cyclic_processes::Model,
        game_id: &i32,
        db_connection: &C,
    ) -> Result<CyclicProcess, DomainError<ProcessErrorKind>> {
        let process_resources: Vec<Resource> = resources::Entity::find()
            .inner_join(cyclic_process_resources::Entity)
            .filter(cyclic_process_resources::Column::CylicProcessId.eq(process_model.id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect();

        let mut assigned_actors: Vec<ActorKind> = Vec::new();

        if let Some(assigned_player) = players::Entity::find()
            .inner_join(games::Entity)
            .filter(games::Column::Id.eq(*game_id))
            .filter(players::Column::AssignedProcessId.eq(process_model.id))
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::FindActors).with_cause(error))?
            .map(|player_model| {
                ActorKind::Player(PlayerMapper::to_domain_entity(player_model, None))
            })
        {
            assigned_actors.push(assigned_player);
        }

        let assigned_monk_models = monks::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                db_connection.get_database_backend(),
                r#"
                        SELECT mo.*
                        FROM monks mo
                        INNER JOIN monastery_monks mm ON mm.monk_id = mo.id
                        INNER JOIN games g ON g.monastery_id = mm.monastery_id
                        WHERE g.id = $1
                        AND mo.assigned_cyclic_process_id = $2
                    "#,
                [(*game_id).into(), process_model.id.into()],
            ))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::FindActors).with_cause(error))?;

        let all_monk_ids: Vec<i32> = assigned_monk_models.iter().map(|model| model.id).collect();

        let all_monk_skill_assignments = monk_skills::Entity::find()
            .filter(monk_skills::Column::MonkId.is_in(all_monk_ids.clone()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::FindActors).with_cause(error))?;

        let all_monk_skills: Vec<Skill> = skills::Entity::find()
            .inner_join(monk_skills::Entity)
            .filter(monk_skills::Column::MonkId.is_in(all_monk_ids))
            .distinct()
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::FindActors).with_cause(error))?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect();

        let assigned_monks: Vec<ActorKind> = assigned_monk_models
            .into_iter()
            .map(|monk_model| {
                let monk_skill_ids: Vec<i32> = all_monk_skill_assignments
                    .iter()
                    .filter(|assignment| assignment.monk_id == monk_model.id)
                    .map(|assignment| assignment.skill_id)
                    .collect();

                let monk_skills: Vec<Skill> = all_monk_skills
                    .clone()
                    .into_iter()
                    .filter(|skill| monk_skill_ids.contains(&skill.id().unwrap()))
                    .collect();

                ActorKind::Monk(
                    MonkMapper::to_domain_entity(monk_model, monk_skills, None).unwrap(),
                )
            })
            .collect();

        assigned_actors = [assigned_actors, assigned_monks].concat();

        CyclicProcessMapper::to_domain_entity(process_model, process_resources, assigned_actors)
    }

    /// Gets all the related entities of a [`CyclicProcess`].
    async fn get_relations<C: ConnectionTrait>(
        &self,
        process_model: cyclic_processes::Model,
        db_connection: &C,
    ) -> Result<CyclicProcess, DomainError<ProcessErrorKind>> {
        let process_resources: Vec<Resource> = resources::Entity::find()
            .inner_join(cyclic_process_resources::Entity)
            .filter(cyclic_process_resources::Column::CylicProcessId.eq(process_model.id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect();

        CyclicProcessMapper::to_domain_entity(process_model, process_resources, Vec::new())
    }

    /// Finds a [`CyclicProcess`] by its ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<CyclicProcess>, DomainError<ProcessErrorKind>> {
        let Some(process_model) = cyclic_processes::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        Ok(Some(CyclicProcessMapper::to_domain_entity(
            process_model,
            Vec::new(),
            Vec::new(),
        )?))
    }

    /// Gets a [`CyclicProcess`] with all its related elements for the provided ID.
    pub async fn get_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<CyclicProcess, DomainError<ProcessErrorKind>> {
        self.find_by_id_with_relations(id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(ProcessErrorKind::GetById))
    }

    /// Finds a [`CyclicProcess`] with all its related entities by its ID.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<CyclicProcess>, DomainError<ProcessErrorKind>> {
        let Some(process_model) = cyclic_processes::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        Ok(Some(
            self.get_relations(process_model, db_connection).await?,
        ))
    }

    /// Finds a [`CyclicProcess`] with all its related entities for the provided ID and Game ID.
    pub async fn find_by_id_for_game_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        game_id: &i32,
        db_connection: &C,
    ) -> Result<Option<CyclicProcess>, DomainError<ProcessErrorKind>> {
        let statement = Statement::from_sql_and_values(
            db_connection.get_database_backend(),
            r#"
                    SELECT cp.*
                    FROM games g
                    INNER JOIN monastery_monks mm ON mm.monastery_id = g.monastery_id
                    INNER JOIN monks mo ON mo.id =  mm.monk_id
                    INNER JOIN cyclic_processes cp ON cp.id = mo.assigned_cyclic_process_id
                    WHERE g.id = $1 AND cp.id = $2 

                    UNION

                    SELECT cp.*
                    FROM games g
                    INNER JOIN players p ON p.id = g.player_id
                    INNER JOIN cyclic_processes cp ON cp.id = p.assigned_process_id
                    WHERE g.id = $1 AND cp.id = $2 

                    UNION

                    SELECT cp.*
                    FROM games g
                    INNER JOIN surroundings_sources ss ON ss.surroundings_id = g.surroundings_id
                    INNER JOIN sources s ON s.id = ss.source_id
                    INNER JOIN cyclic_processes cp ON cp.id = s.cyclic_process_id
                    WHERE g.id = $1 AND cp.id = $2 
            "#,
            [(*game_id).into(), (*id).into()],
        );

        let cyclic_process_model = cyclic_processes::Entity::find()
            .from_raw_sql(statement)
            .one(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::FindByIdForGame).with_cause(error)
            })?
            .ok_or_else(|| DomainError::from(ProcessErrorKind::NotFound))?;

        Ok(Some(
            self.get_relations_for_game(cyclic_process_model, game_id, db_connection)
                .await?,
        ))
    }

    /// Gets a [`CyclicProcess`] with its related entities for the provided ID and Game ID.
    pub async fn get_by_id_for_game_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        game_id: &i32,
        db_connection: &C,
    ) -> Result<CyclicProcess, DomainError<ProcessErrorKind>> {
        self.find_by_id_for_game_with_relations(id, game_id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(ProcessErrorKind::NotFound))
    }

    /// Creates a [`CyclicProcess`] and persists it in the database.
    pub async fn create(
        &self,
        creation_form: CyclicProcessCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<CyclicProcess, DomainError<ProcessErrorKind>> {
        let new_process_model: cyclic_processes::Model =
            CyclicProcessMapper::to_new_active_model(creation_form.clone())
                .insert(db_transaction)
                .await
                .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))?;

        if !creation_form.output_resources_ids.is_empty() {
            let process_resources: Vec<cyclic_process_resources::ActiveModel> = creation_form
                .output_resources_ids
                .iter()
                .map(|resource_id| {
                    CyclicProcessResourceMapper::to_new_active_model(
                        CyclicProcessResourceCreationForm::new(new_process_model.id, *resource_id),
                    )
                })
                .collect();

            cyclic_process_resources::Entity::insert_many(process_resources)
                .exec(db_transaction)
                .await
                .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))?;
        }

        self.find_by_id_with_relations(&new_process_model.id, db_transaction)
            .await?
            .ok_or(DomainError::from(ProcessErrorKind::Creation))
    }

    /// Updates a [`CyclicProcess`].
    pub async fn update(
        &self,
        cyclic_process: CyclicProcess,
        db_transaction: &DatabaseTransaction,
    ) -> Result<CyclicProcess, DomainError<ProcessErrorKind>> {
        cyclic_processes::Entity::update(CyclicProcessMapper::to_update_active_model(
            cyclic_process.clone(),
        ))
        .exec(db_transaction)
        .await
        .map_err(|error| DomainError::from(ProcessErrorKind::Update).with_cause(error))?;

        self.get_by_id_with_relations(&cyclic_process.id().unwrap(), db_transaction)
            .await
    }
}
