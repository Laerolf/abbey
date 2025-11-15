use crate::{
    features::output::{
        domain::resource::{Category, Resource},
        error::ResourceError,
        forms::ResourceCreationForm,
        mapper::ResourceMapper,
        repository::ResourceRepository,
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Resource`] topic.
#[derive(Default)]
pub struct ResourceService {
    repository: ResourceRepository,
}

impl ResourceService {
    /// Finds a [`Resource`] by its name, if not found, the resource will be created.
    pub async fn find_by_name_or_create(
        &self,
        name: impl Into<String>,
        category: Category,
    ) -> Result<Resource, Box<dyn DomainError>> {
        let name_string = name.into();

        if let Ok(Some(existing_resource)) = self.repository.find_by_name(&name_string).await {
            return Ok(ResourceMapper::to_domain_entity(existing_resource));
        }

        self.create_resource(name_string, category).await
    }

    /// Creates a new [`Resource`].
    pub async fn create_resource(
        &self,
        name: impl Into<String>,
        category: Category,
    ) -> Result<Resource, Box<dyn DomainError>> {
        let creation_form = ResourceCreationForm::new(name, category.to_string());

        match self
            .repository
            .insert(ResourceMapper::to_new_active_model(creation_form))
            .await
        {
            Ok(new_resource) => Ok(ResourceMapper::to_domain_entity(new_resource)),
            Err(_error) => Err(Box::new(ResourceError::Creation)),
        }
    }
}
