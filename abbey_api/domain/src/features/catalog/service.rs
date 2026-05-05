use sea_orm::ConnectionTrait;

use crate::{
    features::{
        catalog::error::CatalogErrorKind,
        output::{domain::resource::Resource, repository::ResourceRepository},
        skill::{domain::Skill, repository::SkillRepository},
    },
    shared::error::DomainError,
};

/// Represents a service that handles the catalog topics.
#[derive(Clone)]
pub struct CatalogService {
    skill_repository: SkillRepository,
    resource_repository: ResourceRepository,
}

impl CatalogService {
    /// Creates a new [`CatalogService`].
    pub fn new(skill_repository: SkillRepository, resource_repository: ResourceRepository) -> Self {
        Self {
            skill_repository,
            resource_repository,
        }
    }

    /// Gets all [Skills][Vec<Skill>].
    pub async fn get_all_skills<C: ConnectionTrait>(
        &self,
        db_connection: &C,
    ) -> Result<Vec<Skill>, DomainError<CatalogErrorKind>> {
        self.skill_repository
            .get_all(db_connection)
            .await
            .map_err(|error| DomainError::from(CatalogErrorKind::GetAllSkills).with_cause(error))
    }

    /// Gets all [Resources][Vec<Resource>].
    pub async fn get_all_resources<C: ConnectionTrait>(
        &self,
        db_connection: &C,
    ) -> Result<Vec<Resource>, DomainError<CatalogErrorKind>> {
        self.resource_repository
            .get_all(db_connection)
            .await
            .map_err(|error| DomainError::from(CatalogErrorKind::GetAllResources).with_cause(error))
    }
}
