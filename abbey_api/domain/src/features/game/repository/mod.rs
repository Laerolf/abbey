use entity::{
    cyclic_process_resources, cyclic_processes, games, monasteries, monastery_monks, monk_skills,
    monks, players, resources, skills, sources, surroundings, surroundings_sources,
};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect,
};

use crate::{
    features::{
        actor::mapper::MonkMapper,
        game::{domain::Game, error::GameErrorKind, forms::GameCreationForm, mapper::GameMapper},
        monastery::{domain::Monastery, mapper::MonasteryMapper},
        output::{domain::resource::Resource, mapper::ResourceMapper},
        player::{domain::Player, mapper::PlayerMapper},
        process::{
            domain::{ProcessKind, cyclic_process::CyclicProcess},
            mapper::cyclic_process::CyclicProcessMapper,
        },
        skill::{domain::Skill, mapper::SkillMapper},
        source::{domain::Source, mapper::SourceMapper},
        surroundings::{domain::Surroundings, mapper::SurroundingsMapper},
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents an element that handles all [Game][`crate::features::game::domain::Game`] database topics.
#[derive(Default, Clone)]
pub struct GameRepository;

impl GameRepository {
    /// Gets the [Player], [Monastery] and [Surroundings] of a [`Game`][games::Model].
    async fn get_relations<C: ConnectionTrait>(
        &self,
        game_model: games::Model,
        db_connection: &C,
    ) -> Result<Game, DomainError<GameErrorKind>> {
        let player = get_player(&game_model, db_connection).await?;
        let monastery = get_monastery(&game_model, db_connection).await?;
        let surroundings = get_surroundings(&game_model, db_connection).await?;

        Ok(GameMapper::to_domain_entity(
            game_model,
            player,
            monastery,
            surroundings,
        ))
    }

    /// Creates a new [Game][`games::Model`] and persists it in the database.
    pub async fn create<C: ConnectionTrait>(
        &self,
        creation_form: GameCreationForm,
        db_connection: &C,
    ) -> Result<Game, DomainError<GameErrorKind>> {
        let new_game_model: games::Model = GameMapper::to_new_active_model(creation_form)
            .insert(db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        self.get_relations(new_game_model, db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))
    }

    /// Finds a [Game] by its ID and with all its related entities.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<Game>, DomainError<GameErrorKind>> {
        let Some(game_model) = games::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        Ok(Some(self.get_relations(game_model, db_connection).await?))
    }
}

/// Gets the [Player] of the [`Game`].
async fn get_player<C: ConnectionTrait>(
    game_model: &games::Model,
    db_connection: &C,
) -> Result<Player, DomainError<GameErrorKind>> {
    let Some(player) = players::Entity::find()
        .inner_join(games::Entity)
        .filter(games::Column::PlayerId.eq(game_model.player_id))
        .one(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::PlayerNotFound).with_cause(error))?
        .map(|player_model| PlayerMapper::to_domain_entity(player_model, None))
    else {
        return Err(DomainError::from(GameErrorKind::PlayerNotFound));
    };

    Ok(player)
}

/// Gets the [Monastery] of a [`Game`].
async fn get_monastery<C: ConnectionTrait>(
    game_model: &games::Model,
    db_connection: &C,
) -> Result<Monastery, DomainError<GameErrorKind>> {
    let Some(monastery_model) = monasteries::Entity::find()
        .inner_join(games::Entity)
        .filter(games::Column::MonasteryId.eq(game_model.monastery_id))
        .one(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))?
    else {
        return Err(DomainError::from(GameErrorKind::MonasteryNotFound));
    };

    let monastery_monks_models = monks::Entity::find()
        .inner_join(monastery_monks::Entity)
        .filter(monastery_monks::Column::MonasteryId.eq(monastery_model.id))
        .all(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))?;

    let monastery_monk_ids: Vec<i32> = monastery_monks_models
        .iter()
        .map(|monastery_monk_model| monastery_monk_model.id)
        .collect();

    let monastery_monk_skill_assignments = monk_skills::Entity::find()
        .filter(monk_skills::Column::MonkId.is_in(monastery_monk_ids))
        .all(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))?;

    let monastery_monk_skill_ids: Vec<i32> = monastery_monk_skill_assignments
        .iter()
        .map(|assignment| assignment.skill_id)
        .collect();

    let monastery_monk_skills: Vec<Skill> = skills::Entity::find()
        .filter(skills::Column::Id.is_in(monastery_monk_skill_ids))
        .distinct()
        .all(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))?
        .into_iter()
        .map(SkillMapper::to_domain_entity)
        .collect();

    let monastery_monk_process_ids: Vec<i32> = monastery_monks_models
        .iter()
        .filter_map(|monastery_monk_model| monastery_monk_model.assigned_cyclic_process_id)
        .collect();

    let monastery_monk_processes: Vec<CyclicProcess> = cyclic_processes::Entity::find()
        .filter(cyclic_processes::Column::Id.is_in(monastery_monk_process_ids))
        .all(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))?
        .into_iter()
        .map(|cyclic_process_model| {
            CyclicProcessMapper::to_domain_entity(cyclic_process_model, Vec::new(), Vec::new())
                .unwrap()
        })
        .collect();

    let monastery_monks = monastery_monks_models
        .into_iter()
        .map(|monk_model| {
            let monk_skill_ids: Vec<i32> = monastery_monk_skill_assignments
                .iter()
                .filter(|skill_assignment| skill_assignment.monk_id == monk_model.id)
                .map(|skill_assignment| skill_assignment.skill_id)
                .collect();

            let mut monk_skills: Vec<Skill> = monastery_monk_skills
                .iter()
                .filter(|skill| monk_skill_ids.contains(&skill.id().unwrap()))
                .cloned()
                .collect();

            monk_skills.sort_by_key(|skill| skill.id().unwrap());

            let monk_process = monastery_monk_processes
                .iter()
                .find(|cyclic_process| {
                    cyclic_process.id().unwrap() == monk_model.assigned_cyclic_process_id.unwrap()
                })
                .map(|cyclic_process| ProcessKind::CyclicProcess(cyclic_process.clone()));

            MonkMapper::to_domain_entity(monk_model, monk_skills, monk_process).unwrap()
        })
        .collect();

    MonasteryMapper::to_domain_entity(monastery_model, monastery_monks)
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))
}

/// Gets the [Surroundings] of a [Game].
async fn get_surroundings<C: ConnectionTrait>(
    game_model: &games::Model,
    db_connection: &C,
) -> Result<Surroundings, DomainError<GameErrorKind>> {
    let Some(surroundings_model) = surroundings::Entity::find()
        .inner_join(games::Entity)
        .filter(games::Column::SurroundingsId.eq(game_model.surroundings_id))
        .one(db_connection)
        .await
        .map_err(|error| {
            DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
        })?
    else {
        return Err(DomainError::from(GameErrorKind::SurroundingsNotFound));
    };

    let surroundings_source_models = sources::Entity::find()
        .inner_join(surroundings_sources::Entity)
        .filter(surroundings_sources::Column::SurroundingsId.eq(surroundings_model.id))
        .all(db_connection)
        .await
        .map_err(|error| {
            DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
        })?;

    let surroundings_source_process_ids: Vec<i32> = surroundings_source_models
        .iter()
        .map(|surroundings_source_model| surroundings_source_model.cyclic_process_id)
        .collect();

    let surroundings_source_process_models = cyclic_processes::Entity::find()
        .filter(cyclic_processes::Column::Id.is_in(surroundings_source_process_ids))
        .all(db_connection)
        .await
        .map_err(|error| {
            DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
        })?;

    let source_process_ids: Vec<i32> = surroundings_source_process_models
        .iter()
        .map(|surroundings_source_process_model| surroundings_source_process_model.id)
        .collect();

    let all_source_output_resource_assignments = cyclic_process_resources::Entity::find()
        .filter(cyclic_process_resources::Column::CylicProcessId.is_in(source_process_ids.clone()))
        .all(db_connection)
        .await
        .map_err(|error| {
            DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
        })?;

    let all_source_output_resource_ids: Vec<i32> = all_source_output_resource_assignments
        .iter()
        .map(|source_output_resource_assignment_model| {
            source_output_resource_assignment_model.resource_id
        })
        .collect();

    let all_source_output_resources: Vec<Resource> = resources::Entity::find()
        .filter(resources::Column::Id.is_in(all_source_output_resource_ids))
        .all(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error))?
        .into_iter()
        .map(ResourceMapper::to_domain_entity)
        .collect();

    let surroundings_sources: Vec<Source> = surroundings_source_models
        .into_iter()
        .map(|source_model| {
            let process_model = surroundings_source_process_models
                .iter()
                .find(|surroundings_source_process_model| {
                    surroundings_source_process_model.id == source_model.cyclic_process_id
                })
                .ok_or_else(|| DomainError::from(GameErrorKind::SurroundingsNotFound))
                .unwrap()
                .clone();

            let output_resource_ids: Vec<i32> = all_source_output_resource_assignments
                .iter()
                .filter(|source_output_resource_assignment_model| {
                    source_output_resource_assignment_model.cylic_process_id == process_model.id
                })
                .map(|source_output_resource_assignment_model| {
                    source_output_resource_assignment_model.resource_id
                })
                .collect();

            let output_resources: Vec<Resource> = all_source_output_resources
                .clone()
                .into_iter()
                .filter(|resource| output_resource_ids.contains(&resource.id().unwrap()))
                .collect();

            let process =
                CyclicProcessMapper::to_domain_entity(process_model, output_resources, Vec::new())
                    .unwrap();

            SourceMapper::to_domain_entity(source_model, process)
        })
        .collect();

    SurroundingsMapper::to_domain_entity(surroundings_model, surroundings_sources)
        .map_err(|error| DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error))
}
