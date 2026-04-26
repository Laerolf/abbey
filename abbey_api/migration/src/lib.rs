pub use sea_orm_migration::prelude::*;

mod m20251102_095220_create_cyclic_processes_table;
mod m20251102_100818_create_monasteries_table;
mod m20251102_101143_create_surroundings_table;
mod m20251102_101909_create_resources_table;
mod m20251102_112853_create_skills_table;
mod m20251103_085752_create_players_table;
mod m20251103_091714_create_monks_table;
mod m20251103_231216_create_sources_table;
mod m20251104_051335_create_cyclic_process_resources_table;
mod m20251104_064534_create_monastery_monks_table;
mod m20251104_072921_create_monk_skills_table;
mod m20251105_050258_create_games_table;
mod m20251115_235801_create_users_table;
mod m20251116_235801_create_user_games;
mod m20251122_023003_create_surroundings_sources_table;

mod m20251102_095220_create_tasks_table;
mod m20251104_051335_create_task_input_resources_table;
mod m20251104_051335_create_task_output_resources_table;
mod m20260103_020214_create_refresh_tokens;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251102_095220_create_cyclic_processes_table::Migration),
            Box::new(m20251102_100818_create_monasteries_table::Migration),
            Box::new(m20251102_101143_create_surroundings_table::Migration),
            Box::new(m20251102_101909_create_resources_table::Migration),
            Box::new(m20251102_112853_create_skills_table::Migration),
            Box::new(m20251103_085752_create_players_table::Migration),
            Box::new(m20251103_091714_create_monks_table::Migration),
            Box::new(m20251103_231216_create_sources_table::Migration),
            Box::new(m20251104_051335_create_cyclic_process_resources_table::Migration),
            Box::new(m20251104_064534_create_monastery_monks_table::Migration),
            Box::new(m20251104_072921_create_monk_skills_table::Migration),
            Box::new(m20251105_050258_create_games_table::Migration),
            Box::new(m20251115_235801_create_users_table::Migration),
            Box::new(m20251116_235801_create_user_games::Migration),
            Box::new(m20251122_023003_create_surroundings_sources_table::Migration),
            Box::new(m20260103_020214_create_refresh_tokens::Migration),
            Box::new(m20251102_095220_create_tasks_table::Migration),
            Box::new(m20251104_051335_create_task_output_resources_table::Migration),
            Box::new(m20251104_051335_create_task_input_resources_table::Migration),
        ]
    }
}
