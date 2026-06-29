[AGENT-MANAGED FILE — DO NOT MODIFY MANUALLY]

# Next task
## Title
Implement storage layer

## Description
Implement storage path resolution: use the `TODO_CLI_FILE` environment variable if set,
otherwise resolve `~/.todo-cli/tasks.json` via the `dirs` crate. Implement `load_tasks(path)`
which returns an empty `Vec<Task>` if the file does not exist and propagates any other I/O
error. Implement `save_tasks(path, tasks)` which serializes the task list to JSON and writes
it to the given path, automatically creating the parent directory if it does not exist. Add
unit tests covering: loading from a non-existent path returns an empty vec, and a full
save-then-load round-trip preserves the task list.

## Completion criteria
- Storage path resolves to `TODO_CLI_FILE` env var when set, otherwise `~/.todo-cli/tasks.json`.
- `load_tasks` returns an empty vec for a missing file and propagates other I/O errors.
- `save_tasks` auto-creates the parent directory when absent.
- Unit tests for missing-file load and save/load round-trip pass.
- `cargo test` passes with zero failures.
- `cargo clippy --all-targets --all-features -- -D warnings` reports zero warnings.

## Commit
8e89fb98fada7a8329bd97b169aa7298cf3e78e0
