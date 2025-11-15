pub use sea_orm_migration::prelude::*;

mod m20251103_085752_create_players_table;
mod m20251103_095220_create_cyclic_processes_table;
mod m20251105_050258_create_games_table;
mod m20251105_050818_create_monasteries_table;
mod m20251105_051143_create_surroundings_table;
mod m20251105_071714_create_monks_table;
mod m20251106_231216_create_sources_table;
mod m20251109_050909_create_resources_table;
mod m20251109_051335_create_cyclic_process_resources_table;
mod m20251115_034534_create_monastery_monks_table;
mod m20251115_042853_create_skills_table;
mod m20251115_042921_create_monk_skills_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251103_095220_create_cyclic_processes_table::Migration),
            Box::new(m20251115_042853_create_skills_table::Migration),
            Box::new(m20251109_050909_create_resources_table::Migration),
            Box::new(m20251105_071714_create_monks_table::Migration),
            Box::new(m20251105_050818_create_monasteries_table::Migration),
            Box::new(m20251103_085752_create_players_table::Migration),
            Box::new(m20251105_051143_create_surroundings_table::Migration),
            Box::new(m20251106_231216_create_sources_table::Migration),
            Box::new(m20251109_051335_create_cyclic_process_resources_table::Migration),
            Box::new(m20251115_034534_create_monastery_monks_table::Migration),
            Box::new(m20251115_042921_create_monk_skills_table::Migration),
            Box::new(m20251105_050258_create_games_table::Migration),
        ]
    }
}
