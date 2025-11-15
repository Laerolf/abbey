use entity::monastery_monks;
use futures::future::try_join_all;
use sea_orm::DbErr;

use crate::{
    features::{
        actor::{domain::monk::Monk, service::MonkService},
        monastery::{
            domain::Monastery,
            error::MonasteryError,
            forms::MonasteryCreationForm,
            mapper::MonasteryMapper,
            repository::{MonasteryMonksRepository, MonasteryRepository},
        },
    },
    shared::error::DomainError,
};

/// The default amount of [Monks][`Monk`] in [`Monastery`].
pub const DEFAULT_AMOUNT_OF_MONKS: i32 = 10;

/// Represents a service handling the [`Monastery`] topic.
#[derive(Clone)]
pub struct MonasteryService {
    repository: MonasteryRepository,
    monastery_monks_repository: MonasteryMonksRepository,
    monk_service: MonkService,
}

impl MonasteryService {
    /// Creates a new [`MonasteryService`].
    pub fn new(monk_service: MonkService) -> Self {
        Self {
            repository: MonasteryRepository::default(),
            monastery_monks_repository: MonasteryMonksRepository::default(),
            monk_service,
        }
    }

    /// Adds a [Monk] to a [`Monastery`].
    async fn add_monk_to_monastery(
        &self,
        monastery: &Monastery,
        monk: &Monk,
    ) -> Result<monastery_monks::Model, DbErr> {
        self.monastery_monks_repository
            .insert(MonasteryMapper::to_new_monastery_monk_active_model(
                monastery, monk,
            ))
            .await
    }

    /// Creates [Monks][`Monk`] for a new [`Monastery`].
    async fn create_monks(&self) -> Result<Vec<Monk>, Box<dyn DomainError>> {
        let monks =
            try_join_all((0..DEFAULT_AMOUNT_OF_MONKS).map(|_| self.monk_service.create_monk()))
                .await?;

        Ok(monks)
    }

    /// Creates a new [`Monastery`].
    pub async fn create_monastery(&self) -> Result<Monastery, Box<dyn DomainError>> {
        let monks: Vec<Monk> = self
            .create_monks()
            .await
            .expect("Failed to create Monks for a new Monastery.");

        let creation_form = MonasteryCreationForm::new();

        match self
            .repository
            .insert(MonasteryMapper::to_new_active_model(creation_form))
            .await
        {
            Ok(new_monastery) => {
                let monastery = MonasteryMapper::to_domain_entity(new_monastery, monks.clone());

                match try_join_all(
                    monks
                        .iter()
                        .map(|monk| self.add_monk_to_monastery(&monastery, monk)),
                )
                .await
                {
                    Ok(_) => Ok(monastery),
                    Err(_) => Err(Box::new(MonasteryError::Creation)),
                }
            }
            Err(_error) => Err(Box::new(MonasteryError::Creation)),
        }
    }
}
