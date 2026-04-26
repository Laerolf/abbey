use entity::{games, players, resources, task_input_resources, task_output_resources, tasks};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter,
    Statement,
};

use crate::{
    features::{
        actor::domain::ActorKind,
        output::{domain::resource::Resource, mapper::ResourceMapper},
        player::mapper::PlayerMapper,
        process::{
            domain::{Process, task::Task},
            error::ProcessErrorKind,
            forms::task::{
                TaskCreationForm, TaskInputResourceCreationForm, TaskOutputResourceCreationForm,
            },
            mapper::task::{TaskInputResourceMapper, TaskMapper, TaskOutputResourceMapper},
        },
    },
    shared::error::DomainError,
};

/// Represents an element that handles all [Task] database topics.
#[derive(Default, Clone)]
pub struct TaskRepository;

impl TaskRepository {
    /// Creates a [`Task`] and persists it in the database.
    pub async fn create(
        &self,
        creation_form: TaskCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Task, DomainError<ProcessErrorKind>> {
        let new_task_model: tasks::Model = TaskMapper::to_new_active_model(creation_form.clone())
            .insert(db_transaction)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))?;

        let input_resources: Vec<task_input_resources::ActiveModel> = creation_form
            .output_resources_ids
            .iter()
            .map(|resource_id| {
                TaskInputResourceMapper::to_new_active_model(TaskInputResourceCreationForm::new(
                    new_task_model.id,
                    *resource_id,
                ))
            })
            .collect();

        task_input_resources::Entity::insert_many(input_resources)
            .exec(db_transaction)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))?;

        let output_resources: Vec<task_output_resources::ActiveModel> = creation_form
            .output_resources_ids
            .iter()
            .map(|resource_id| {
                TaskOutputResourceMapper::to_new_active_model(TaskOutputResourceCreationForm::new(
                    new_task_model.id,
                    *resource_id,
                ))
            })
            .collect();

        task_output_resources::Entity::insert_many(output_resources)
            .exec(db_transaction)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))?;

        self.find_by_id_with_relations(&new_task_model.id, db_transaction)
            .await?
            .ok_or(DomainError::from(ProcessErrorKind::Creation))
    }

    /// Gets all the related entities of a [`Task`] for a Game.
    async fn get_relations_for_game<C: ConnectionTrait>(
        &self,
        task_model: tasks::Model,
        game_id: &i32,
        db_connection: &C,
    ) -> Result<Task, DomainError<ProcessErrorKind>> {
        let task_input_resources: Vec<Resource> = resources::Entity::find()
            .inner_join(task_input_resources::Entity)
            .filter(task_input_resources::Column::TaskId.eq(task_model.id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect();

        let task_output_resources: Vec<Resource> = resources::Entity::find()
            .inner_join(task_output_resources::Entity)
            .filter(task_output_resources::Column::TaskId.eq(task_model.id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect();

        let mut assigned_people: Vec<ActorKind> = Vec::new();

        if let Some(assigned_player) = players::Entity::find()
            .inner_join(games::Entity)
            .filter(games::Column::Id.eq(*game_id))
            .filter(players::Column::AssignedProcessId.eq(task_model.id))
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::FindActors).with_cause(error))?
            .map(|player_model| {
                ActorKind::Player(PlayerMapper::to_domain_entity(player_model, None))
            })
        {
            assigned_people.push(assigned_player);
        }

        TaskMapper::to_domain_entity(
            task_model,
            task_input_resources,
            task_output_resources,
            assigned_people,
        )
    }

    /// Gets all the related entities of a [`Task`].
    async fn get_relations<C: ConnectionTrait>(
        &self,
        task_model: tasks::Model,
        db_connection: &C,
    ) -> Result<Task, DomainError<ProcessErrorKind>> {
        let task_input_resources: Vec<Resource> = resources::Entity::find()
            .inner_join(task_input_resources::Entity)
            .filter(task_input_resources::Column::TaskId.eq(task_model.id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect();

        let task_output_resources: Vec<Resource> = resources::Entity::find()
            .inner_join(task_output_resources::Entity)
            .filter(task_output_resources::Column::TaskId.eq(task_model.id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect();

        let mut assigned_people: Vec<ActorKind> = Vec::new();

        if let Some(assigned_player) = players::Entity::find()
            .inner_join(games::Entity)
            .filter(players::Column::AssignedProcessId.eq(task_model.id))
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::FindActors).with_cause(error))?
            .map(|player_model| {
                ActorKind::Player(PlayerMapper::to_domain_entity(player_model, None))
            })
        {
            assigned_people.push(assigned_player);
        }

        TaskMapper::to_domain_entity(
            task_model,
            task_input_resources,
            task_output_resources,
            assigned_people,
        )
    }

    /// Finds a [`Task`] with all its related entities for the provided ID and Game ID.
    pub async fn find_by_id_for_game_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        game_id: &i32,
        db_connection: &C,
    ) -> Result<Option<Task>, DomainError<ProcessErrorKind>> {
        let statement = Statement::from_sql_and_values(
            db_connection.get_database_backend(),
            r#"
                    SELECT t.*
                    FROM games g
                    INNER JOIN players p ON p.id = g.player_id
                    INNER JOIN tasks t ON t.id = p.assigned_process_id
                    WHERE g.id = $1 AND p.assigned_process_id = $2 
            "#,
            [(*game_id).into(), (*id).into()],
        );

        let task_model = tasks::Entity::find()
            .from_raw_sql(statement)
            .one(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::FindByIdForGame).with_cause(error)
            })?
            .ok_or_else(|| DomainError::from(ProcessErrorKind::NotFound))?;

        Ok(Some(
            self.get_relations_for_game(task_model, game_id, db_connection)
                .await?,
        ))
    }

    /// Gets a [`Task`] with all its related entities for the provided ID and Game ID.
    pub async fn get_by_id_for_game_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        game_id: &i32,
        db_connection: &C,
    ) -> Result<Task, DomainError<ProcessErrorKind>> {
        self.find_by_id_for_game_with_relations(id, game_id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(ProcessErrorKind::GetById))
    }

    /// Finds a [`Task`] with all its related entities for the provided ID.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<Task>, DomainError<ProcessErrorKind>> {
        let statement = Statement::from_sql_and_values(
            db_connection.get_database_backend(),
            r#"
                    SELECT t.*
                    FROM games g
                    INNER JOIN players p ON p.id = g.player_id
                    INNER JOIN tasks t ON t.id = p.assigned_process_id
                    WHERE p.assigned_process_id = $2 
            "#,
            [(*id).into()],
        );

        let task_model = tasks::Entity::find()
            .from_raw_sql(statement)
            .one(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::FindByIdForGame).with_cause(error)
            })?
            .ok_or_else(|| DomainError::from(ProcessErrorKind::NotFound))?;

        Ok(Some(self.get_relations(task_model, db_connection).await?))
    }

    /// Gets a [`Task`] with all its related entities for the provided ID.
    pub async fn get_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Task, DomainError<ProcessErrorKind>> {
        self.find_by_id_with_relations(id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(ProcessErrorKind::GetById))
    }

    /// Updates a [`Task`].
    pub async fn update(
        &self,
        task: Task,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Task, DomainError<ProcessErrorKind>> {
        tasks::Entity::update(TaskMapper::to_update_active_model(task.clone()))
            .exec(db_transaction)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Update).with_cause(error))?;

        self.get_by_id_with_relations(&task.id().unwrap(), db_transaction)
            .await
    }
}
