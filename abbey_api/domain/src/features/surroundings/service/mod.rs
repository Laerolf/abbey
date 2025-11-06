use crate::{
    features::surroundings::{
        domain::Surroundings, error::SurroundingsError, forms::SurroundingsCreationForm,
        mapper::SurroundingsMapper, repository::SurroundingsRepository,
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Surroundings`] topic.
#[derive(Default)]
pub struct SurroundingsService {
    repository: SurroundingsRepository,
    mapper: SurroundingsMapper,
}

impl SurroundingsService {
    /// Creates a new [`Surroundings`].
    pub async fn create_surroundings(&self) -> Result<Surroundings, Box<dyn DomainError>> {
        let creation_form = SurroundingsCreationForm::new();

        match self
            .repository
            .insert(self.mapper.to_new_active_model(creation_form))
            .await
        {
            Ok(new_surroundings) => Ok(self.mapper.to_domain_entity(new_surroundings, Vec::new())),
            Err(_error) => Err(Box::new(SurroundingsError::Creation)),
        }
    }
}
