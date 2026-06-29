[AGENT-MANAGED FILE — DO NOT MODIFY MANUALLY]

# Next task
## Title
Define task data model

## Description
Define the `Task` struct with fields `id: u64`, `title: String`, `done: bool`, deriving
`serde::Serialize` and `serde::Deserialize`. Add a `#[cfg(test)]` module with unit tests
covering: JSON serialization round-trip for a single task, and JSON serialization round-trip
for a list of tasks.

## Completion criteria
- `Task` struct is defined with the correct field types and serde derives.
- Unit tests for single-task and list round-trip serialization pass.
- `cargo test` passes with zero failures.
- `cargo clippy --all-targets --all-features -- -D warnings` reports zero warnings.

## Commit
b25f89c0dca3e7b6237bb2bfdf82c33c6e6d9b05
