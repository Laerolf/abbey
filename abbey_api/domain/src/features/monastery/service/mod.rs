use crate::{
    features::monastery::{
        domain::Monastery, error::MonasteryError, forms::MonasteryCreationForm,
        mapper::MonasteryMapper, repository::MonasteryRepository,
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Monastery`] topic.
#[derive(Default)]
pub struct MonasteryService {
    repository: MonasteryRepository,
    mapper: MonasteryMapper,
}

impl MonasteryService {
    /// Creates a new [`MonasteryService`].
    pub fn new() -> Self {
        Self {
            repository: MonasteryRepository::default(),
            mapper: MonasteryMapper::default(),
        }
    }

    /// Creates a new [`Monastery`].
    pub async fn create_monastery(&self) -> Result<Monastery, Box<dyn DomainError>> {
        let creation_form = MonasteryCreationForm::new();

        match self
            .repository
            .insert(self.mapper.to_new_active_model(creation_form))
            .await
        {
            Ok(new_monastery) => Ok(self.mapper.to_domain_entity(new_monastery)),
            Err(_error) => Err(Box::new(MonasteryError::Creation)),
        }
    }
}
