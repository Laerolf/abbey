use entity::{cyclic_process_resources, cyclic_processes};
use sea_orm::{ActiveModelTrait, DbErr};

use crate::shared::db::DatabasePool;

/// Represents an element that handles all [CyclicProcess][`super::domain::CyclicProcess`] database topics.
#[derive(Default, Clone)]
pub struct CyclicProcessRepository {}

impl CyclicProcessRepository {
    /// Inserts a [CyclicProcess][`cyclic_processes::Model`].
    pub async fn insert(
        &self,
        new_cyclic_process: cyclic_processes::ActiveModel,
    ) -> Result<cyclic_processes::Model, DbErr> {
        new_cyclic_process.insert(DatabasePool::instance()).await
    }
}

/// Represents an element that handles all [CyclicProcessResource][`cyclic_process_resources::Model`] database topics.
#[derive(Default, Clone)]
pub struct CyclicProcessResourceRepository {}

impl CyclicProcessResourceRepository {
    /// Inserts a [CyclicProcessResource][`cyclic_process_resources::Model`].
    pub async fn insert(
        &self,
        new_cyclic_process_resource: cyclic_process_resources::ActiveModel,
    ) -> Result<cyclic_process_resources::Model, DbErr> {
        new_cyclic_process_resource
            .insert(DatabasePool::instance())
            .await
    }
}
