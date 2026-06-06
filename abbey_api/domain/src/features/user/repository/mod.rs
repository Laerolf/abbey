use entity::{
    cyclic_process_resources, cyclic_processes, games, monasteries, monastery_monks, monk_skills,
    monks, players, resources, skills, sources, surroundings, surroundings_sources, user_games,
    users,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Statement,
};

use crate::{
    features::{
        actor::{domain::monk::Monk, mapper::MonkMapper},
        game::{domain::Game, mapper::GameMapper},
        monastery::mapper::MonasteryMapper,
        output::{domain::resource::Resource, mapper::ResourceMapper},
        player::mapper::PlayerMapper,
        process::{
            domain::cyclic_process::CyclicProcess, mapper::cyclic_process::CyclicProcessMapper,
        },
        skill::{domain::Skill, mapper::SkillMapper},
        source::{domain::Source, mapper::SourceMapper},
        surroundings::mapper::SurroundingsMapper,
        user::{domain::User, error::UserErrorKind, mapper::UserMapper},
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents an element that handles all [`User`] database topics.
#[derive(Default, Clone)]
pub struct UserRepository;

impl UserRepository {
    /// Gets all [`Game`]s for a User ID.
    async fn get_all_games_by_user_id_with_relations<C: ConnectionTrait>(
        &self,
        user_id: &i32,
        connection: &C,
    ) -> Result<Vec<Game>, DomainError<UserErrorKind>> {
        let games_and_players = games::Entity::find()
            .inner_join(user_games::Entity)
            .filter(user_games::Column::UserId.eq(*user_id))
            .find_also_related(players::Entity)
            .all(connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
            })?;

        let game_ids: Vec<i32> = games_and_players.iter().map(|(game, _)| game.id).collect();

        let monasteries = monasteries::Entity::find()
            .inner_join(games::Entity)
            .filter(games::Column::Id.is_in(game_ids.clone()))
            .all(connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
            })?;

        let all_surroundings_models = surroundings::Entity::find()
            .inner_join(games::Entity)
            .filter(games::Column::Id.is_in(game_ids))
            .all(connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
            })?;

        let all_surroundings_ids: Vec<i32> = all_surroundings_models
            .iter()
            .map(|surroundings_model| surroundings_model.id)
            .collect();

        let all_surroundings_source_models = sources::Entity::find()
            .inner_join(surroundings_sources::Entity)
            .filter(sources::Column::Id.is_in(all_surroundings_ids))
            .all(connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
            })?;

        let all_surroundings_source_process_ids: Vec<i32> = all_surroundings_source_models
            .iter()
            .map(|surroundings_source_model| surroundings_source_model.cyclic_process_id)
            .collect();

        let all_surroundings_source_processes_models = cyclic_processes::Entity::find()
            .filter(cyclic_processes::Column::Id.is_in(all_surroundings_source_process_ids.clone()))
            .all(connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
            })?;

        let all_surroundings_source_process_output_resource_assignments =
            cyclic_process_resources::Entity::find()
                .filter(
                    cyclic_process_resources::Column::CyclicProcessId
                        .is_in(all_surroundings_source_process_ids.clone()),
                )
                .all(connection)
                .await
                .map_err(|error| {
                    DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
                })?;

        let all_surroundings_source_process_output_resources: Vec<Resource> =
            resources::Entity::find()
                .inner_join(cyclic_process_resources::Entity)
                .filter(
                    cyclic_process_resources::Column::CyclicProcessId
                        .is_in(all_surroundings_source_process_ids),
                )
                .all(connection)
                .await
                .map_err(|error| {
                    DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
                })?
                .into_iter()
                .map(ResourceMapper::to_domain_entity)
                .collect();

        let all_surroundings_source_processes: Vec<CyclicProcess> =
            all_surroundings_source_processes_models
                .into_iter()
                .map(|surroundings_source_process_model| {
                    let source_output_resource_ids: Vec<i32> =
                        all_surroundings_source_process_output_resource_assignments
                            .iter()
                            .filter(|resource_assignment_model| {
                                resource_assignment_model.cyclic_process_id
                                    == surroundings_source_process_model.id
                            })
                            .map(|resource_assignment_model| resource_assignment_model.resource_id)
                            .collect();

                    let source_output_resources = all_surroundings_source_process_output_resources
                        .clone()
                        .into_iter()
                        .filter(|resource| {
                            source_output_resource_ids.contains(&resource.id().unwrap())
                        })
                        .collect();

                    CyclicProcessMapper::to_domain_entity(
                        surroundings_source_process_model,
                        source_output_resources,
                        Vec::new(),
                    )
                    .map_err(|error| {
                        DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
                    })
                })
                .collect::<Result<Vec<CyclicProcess>, DomainError<UserErrorKind>>>()?;

        let all_surroundings_sources: Vec<Source> = all_surroundings_source_models
            .into_iter()
            .map(|source_model| {
                let source_process = all_surroundings_source_processes
                    .iter()
                    .find(|process| source_model.cyclic_process_id == process.id().unwrap())
                    .ok_or_else(|| DomainError::from(UserErrorKind::GetAllGamesByUserId))
                    .unwrap();

                SourceMapper::to_domain_entity(source_model, source_process.clone())
            })
            .collect();

        let all_surroundings_source_assignments = surroundings_sources::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                connection.get_database_backend(),
                r#"
                    SELECT s_s.*
                    FROM surroundings_sources s_s
                    JOIN games g ON g.surroundings_id = s_s.surroundings_id
                    JOIN user_games u_g ON u_g.game_id = g.id
                    WHERE u_g.user_id = $1
                "#,
                [(*user_id).into()],
            ))
            .all(connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
            })?;

        let all_monastery_monks_models = monks::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                connection.get_database_backend(),
                r#"
                    SELECT m.*
                    FROM monks m
                    JOIN monastery_monks m_m ON m_m.monk_id = m.id
                    JOIN games g ON g.monastery_id = m_m.monastery_id
                    JOIN user_games u_g ON u_g.game_id = g.id
                    WHERE u_g.user_id = $1
                "#,
                [(*user_id).into()],
            ))
            .all(connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
            })?;

        let all_monastery_monk_assignments = monastery_monks::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                connection.get_database_backend(),
                r#"
                    SELECT m_m.*
                    FROM monastery_monks m_m
                    JOIN games g ON g.monastery_id = m_m.monastery_id
                    JOIN user_games u_g ON u_g.game_id = g.id
                    WHERE u_g.user_id = $1
                "#,
                [(*user_id).into()],
            ))
            .all(connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
            })?;

        let all_monastery_monk_ids: Vec<i32> = all_monastery_monks_models
            .iter()
            .map(|monk_model| monk_model.id)
            .collect();

        let all_monk_skill_assignments = monk_skills::Entity::find()
            .filter(monk_skills::Column::MonkId.is_in(all_monastery_monk_ids.clone()))
            .all(connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
            })?;

        let all_monastery_monk_skills: Vec<Skill> = skills::Entity::find()
            .inner_join(monk_skills::Entity)
            .filter(monk_skills::Column::MonkId.is_in(all_monastery_monk_ids))
            .distinct()
            .order_by_asc(skills::Column::Id)
            .all(connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
            })?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect();

        let all_monks: Vec<Monk> = all_monastery_monks_models
            .into_iter()
            .map(|monk_model| {
                let monk_skill_ids: Vec<i32> = all_monk_skill_assignments
                    .iter()
                    .filter(|assignment| assignment.monk_id == monk_model.id)
                    .map(|assignment| assignment.skill_id)
                    .collect();

                let monk_skills: Vec<Skill> = all_monastery_monk_skills
                    .clone()
                    .into_iter()
                    .filter(|skill| monk_skill_ids.contains(&skill.id().unwrap()))
                    .collect();

                MonkMapper::to_domain_entity(monk_model, monk_skills, None).unwrap()
            })
            .collect();

        let result = games_and_players
            .into_iter()
            .filter_map(|(game, player)| {
                let player_model = player?;

                let monastery_model = monasteries
                    .iter()
                    .find(|monastery| monastery.id == game.monastery_id)?
                    .clone();

                let monastery_monk_ids: Vec<i32> = all_monastery_monk_assignments
                    .iter()
                    .filter(|assignment| assignment.monastery_id == monastery_model.id)
                    .map(|assignment| assignment.monk_id)
                    .collect();

                let monastery_monks = all_monks
                    .clone()
                    .into_iter()
                    .filter(|monk| monastery_monk_ids.contains(&monk.id().unwrap()))
                    .collect();

                let surroundings_model = all_surroundings_models
                    .iter()
                    .find(|surroundings| surroundings.id == game.surroundings_id)?
                    .clone();

                let surroundings_source_ids: Vec<i32> = all_surroundings_source_assignments
                    .iter()
                    .filter(|assignment| assignment.surroundings_id == surroundings_model.id)
                    .map(|assignment| assignment.source_id)
                    .collect();

                let surroundings_sources: Vec<Source> = all_surroundings_sources
                    .clone()
                    .into_iter()
                    .filter(|source| surroundings_source_ids.contains(&source.id().unwrap()))
                    .collect();

                Some(GameMapper::to_domain_entity(
                    game,
                    PlayerMapper::to_domain_entity(player_model, None),
                    MonasteryMapper::to_domain_entity(monastery_model, monastery_monks)
                        .map_err(|error| {
                            DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
                        })
                        .unwrap(),
                    SurroundingsMapper::to_domain_entity(surroundings_model, surroundings_sources)
                        .map_err(|error| {
                            DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
                        })
                        .unwrap(),
                ))
            })
            .collect();

        Ok(result)
    }

    /// Finds a [`User`][users::Model] by its ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<users::Model>, DomainError<UserErrorKind>> {
        users::Entity::find()
            .filter(users::Column::Id.eq(*id))
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(UserErrorKind::FindById).with_cause(error))
    }

    /// Gets the [`User Games`][user_games::Model] for the provided User ID.
    pub async fn get_user_games_by_user_id<C: ConnectionTrait>(
        &self,
        user_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<user_games::Model>, DomainError<UserErrorKind>> {
        user_games::Entity::find()
            .filter(user_games::Column::UserId.eq(*user_id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::GetAllGamesByUserId).with_cause(error)
            })
    }

    /// Finds a [`User`] by its ID.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        connection: &C,
    ) -> Result<Option<User>, DomainError<UserErrorKind>> {
        let Some(user_model) = users::Entity::find()
            .filter(users::Column::Id.eq(*id))
            .one(connection)
            .await
            .map_err(|error| DomainError::from(UserErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        let user_games = self
            .get_all_games_by_user_id_with_relations(id, connection)
            .await
            .map_err(|error| DomainError::from(UserErrorKind::FindById).with_cause(error))?;

        Ok(Some(UserMapper::to_domain_entity(user_model, user_games)))
    }

    /// Finds a [`User`][users::Model] by its email.
    pub async fn find_by_email<C: ConnectionTrait>(
        &self,
        email: &String,
        connection: &C,
    ) -> Result<Option<users::Model>, DomainError<UserErrorKind>> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(connection)
            .await
            .map_err(|error| DomainError::from(UserErrorKind::FindByEmail).with_cause(error))
    }

    /// Creates a new [`User`] and persists it in the database.
    pub async fn create<C: ConnectionTrait>(
        &self,
        user: users::ActiveModel,
        db_connection: &C,
    ) -> Result<users::Model, DomainError<UserErrorKind>> {
        user.insert(db_connection)
            .await
            .map_err(|error| DomainError::from(UserErrorKind::Insert).with_cause(error))
    }

    /// Updates a [`User`].
    pub async fn update_with_relations(
        &self,
        user: User,
        db_transaction: &DatabaseTransaction,
    ) -> Result<User, DomainError<UserErrorKind>> {
        let user = UserMapper::to_update_active_model(user)
            .update(db_transaction)
            .await
            .map_err(|error| DomainError::from(UserErrorKind::Update).with_cause(error))?;

        self.find_by_id_with_relations(&user.id, db_transaction)
            .await?
            .ok_or(DomainError::from(UserErrorKind::Update))
    }

    /// Assign a Game to a [`User`][user_games::Model].
    pub async fn assign_game<C: ConnectionTrait>(
        &self,
        user_game_model: user_games::ActiveModel,
        db_connection: &C,
    ) -> Result<user_games::Model, DomainError<UserErrorKind>> {
        user_game_model
            .insert(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(UserErrorKind::Creation(
                    super::error::UserCreationErrorKind::AssignGame,
                ))
                .with_cause(error)
            })
    }
}
