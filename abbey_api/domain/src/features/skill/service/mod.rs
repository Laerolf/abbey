use crate::{
    features::skill::{
        domain::Skill, error::SkillError, forms::SkillCreationForm, mapper::SkillMapper,
        repository::SkillRepository,
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Skill`] topic.
#[derive(Default)]
pub struct SkillService {
    repository: SkillRepository,
}

impl SkillService {
    /// Finds a [`Skill`] by its name, if not found, the skill will be created.
    pub async fn find_by_name_or_create(
        &self,
        name: impl Into<String>,
    ) -> Result<Skill, Box<dyn DomainError>> {
        let name_string = name.into();

        if let Ok(Some(existing_resource)) = self.repository.find_by_name(&name_string).await {
            return Ok(SkillMapper::to_domain_entity(existing_resource));
        }

        self.create_skill(name_string).await
    }

    /// Creates a new [`Skill`].
    pub async fn create_skill(
        &self,
        name: impl Into<String>,
    ) -> Result<Skill, Box<dyn DomainError>> {
        let creation_form = SkillCreationForm::new(name);

        match self
            .repository
            .insert(SkillMapper::to_new_active_model(creation_form))
            .await
        {
            Ok(new_skill) => Ok(SkillMapper::to_domain_entity(new_skill)),
            Err(_error) => Err(Box::new(SkillError::Creation)),
        }
    }
}
