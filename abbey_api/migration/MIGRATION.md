# Migration dependencies

## Dependency graph

### Base
- [m20251103_095220_create_cyclic_processes_table.rs](./src/m20251103_095220_create_cyclic_processes_table.rs)
- [m20251105_050818_create_monasteries_table.rs](./src/m20251105_050818_create_monasteries_table.rs)
- [m20251105_051143_create_surroundings_table.rs](./src/m20251105_051143_create_surroundings_table.rs)
- [m20251109_050909_create_resources_table.rs](./src/m20251109_050909_create_resources_table.rs)
- [m20251115_042853_create_skills_table.rs](./src/m20251115_042853_create_skills_table.rs)

### With dependency on Cyclic Process
- [m20251103_085752_create_players_table.rs](./src/m20251103_085752_create_players_table.rs)
- [m20251105_071714_create_monks_table.rs](./src/m20251105_071714_create_monks_table.rs)
- [m20251106_231216_create_sources_table.rs](./src/m20251106_231216_create_sources_table.rs)

### With dependency on Surroundings and Sources
- [m20251122_023003_create_surroundings_sources_table.rs](./src/m20251122_023003_create_surroundings_sources_table.rs)

### With dependency on Cyclic Process and Resource
- [m20251109_051335_create_cyclic_process_resources_table.rs](./src/m20251109_051335_create_cyclic_process_resources_table.rs)

### With dependency on Monastery and Monk
- [m20251115_034534_create_monastery_monks_table.rs](./src/m20251115_034534_create_monastery_monks_table.rs)

### With dependency on Monk and Skill
- [m20251115_042921_create_monk_skills_table.rs](./src/m20251115_042921_create_monk_skills_table.rs)

### With dependency on Player, Monastery and Surroundings
- [m20251105_050258_create_games_table.rs](./src/m20251105_050258_create_games_table.rs)

### With dependency on Game
- [m20251115_235801_create_users_table.rs](./src/m20251115_235801_create_users_table.rs)