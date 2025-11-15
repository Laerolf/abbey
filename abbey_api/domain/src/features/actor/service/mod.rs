use futures::future::try_join_all;

use crate::{
    features::{
        actor::{
            domain::monk::Monk, error::ActorError, forms::MonkCreationForm, mapper::MonkMapper,
            repository::MonkRepository,
        },
        skill::{domain::Skill, service::SkillService},
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Monk`] topic.
#[derive(Clone)]
pub struct MonkService {
    repository: MonkRepository,
    skill_service: SkillService,
}

impl MonkService {
    /// Creates a new [`MonkService`].
    pub fn new(skill_service: SkillService) -> Self {
        Self {
            repository: MonkRepository::default(),
            skill_service,
        }
    }

    /// Creates a [Skill] set for a new [`Monk`].
    pub async fn create_skills(&self) -> Result<Vec<Skill>, Box<dyn DomainError>> {
        let skills = try_join_all(vec![
            self.skill_service.find_by_name_or_create("cooking"),
            self.skill_service.find_by_name_or_create("brewing"),
        ])
        .await?;

        Ok(skills)
    }

    /// Creates a new [`Monk`].
    pub async fn create_monk(&self) -> Result<Monk, Box<dyn DomainError>> {
        let skills: Vec<Skill> = self
            .create_skills()
            .await
            .expect("Failed to create Skills for a Monk.");

        let creation_form =
            MonkCreationForm::new("Maurits", skills.iter().map(|skill| skill.id).collect());

        match self
            .repository
            .insert(MonkMapper::to_new_active_model(creation_form))
            .await
        {
            Ok(new_monk) => Ok(MonkMapper::to_domain_entity(new_monk, skills, None)),
            Err(_error) => Err(Box::new(ActorError::Creation)),
        }
    }
}
