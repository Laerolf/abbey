use sea_orm::ConnectionTrait;

use crate::{
    features::{
        catalog::error::CatalogErrorKind,
        output::{
            domain::resource::Resource, mapper::ResourceMapper, repository::ResourceRepository,
        },
        skill::{domain::Skill, mapper::SkillMapper, repository::SkillRepository},
    },
    shared::error::DomainError,
};

/// Represents a query service for Catalog.
#[derive(Clone)]
pub struct CatalogQueryService {
    skill_repository: SkillRepository,
    resource_repository: ResourceRepository,
}

impl CatalogQueryService {
    /// Creates a new [`CatalogQueryService`].
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
        Ok(self
            .skill_repository
            .get_all(db_connection)
            .await
            .map_err(|error| DomainError::from(CatalogErrorKind::GetAllSkills).with_cause(error))?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect())
    }

    /// Gets all [Resources][Vec<Resource>].
    pub async fn get_all_resources<C: ConnectionTrait>(
        &self,
        db_connection: &C,
    ) -> Result<Vec<Resource>, DomainError<CatalogErrorKind>> {
        Ok(self
            .resource_repository
            .get_all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(CatalogErrorKind::GetAllResources).with_cause(error)
            })?
            .into_iter()
            .map(ResourceMapper::to_domain_entity)
            .collect())
    }
}
