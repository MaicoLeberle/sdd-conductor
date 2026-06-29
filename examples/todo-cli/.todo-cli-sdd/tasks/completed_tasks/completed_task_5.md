[AGENT-MANAGED FILE — DO NOT MODIFY MANUALLY]

# Next task
## Title
Extract data model and storage into `src/tasks.rs`

## Description
Move the `Task` struct (with serde derives) and the storage functions
(`resolve_storage_path`, `load_tasks`, `save_tasks`) from `src/main.rs` into a new
`src/tasks.rs` module. Make all moved items `pub`. Update `src/main.rs` to declare
`mod tasks;` and import the moved items. Move the unit tests covering those items into
`src/tasks.rs`. `src/main.rs` retains only the CLI scaffold (`Cli`, `Commands`),
`format_task_list`, and `main()`.

## Completion criteria
- `src/tasks.rs` exists and contains `Task`, `resolve_storage_path`, `load_tasks`,
  `save_tasks`, and their unit tests.
- `src/main.rs` declares `mod tasks;` and contains only CLI and formatting logic.
- All existing tests continue to pass in their new location.
- `cargo test` passes with zero failures.
- `cargo clippy --all-targets --all-features -- -D warnings` reports zero warnings.

## Commit
9fbf0c7d06b218dcbf0e2810e6506c5b00f1f951
