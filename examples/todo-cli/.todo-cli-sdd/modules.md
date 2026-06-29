[AGENT-MANAGED FILE — DO NOT MODIFY MANUALLY]

<mark>THIS FILE CONTAINS THE DOCUMENTATION OF THE PROJECT MODULE
BOUNDARIES.</mark>


## *MODULES*

### `tasks` (`src/tasks.rs`)
Owns the `Task` struct and all storage logic. Public surface:
- `Task` — struct with fields `id: u64`, `title: String`, `done: bool`
- `resolve_storage_path() -> PathBuf` — resolves the JSON file path from env or default
- `load_tasks(path: &PathBuf) -> Result<Vec<Task>, io::Error>` — deserializes from disk
- `save_tasks(path: &PathBuf, tasks: &[Task]) -> Result<(), io::Error>` — serializes to disk

### `main` (`src/main.rs`)
CLI entry point. Owns the clap scaffold (`Cli`, `Commands`) and output formatting
(`format_task_list`). Depends on `tasks` for all data and I/O operations.
