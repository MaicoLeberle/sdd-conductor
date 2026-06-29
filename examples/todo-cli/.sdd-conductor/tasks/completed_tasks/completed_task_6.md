[AGENT-MANAGED FILE — DO NOT MODIFY MANUALLY]

# Next task
## Title
Implement `add` with ID assignment

## Description
Implement the `add "<title>"` subcommand. Compute the next ID as `max(existing ids) + 1`,
starting from 1 for an empty list; IDs of deleted tasks must never be reused. Create the new
task with `done: false` and save. Add unit tests for: ID assignment from an empty list,
ID assignment from a non-empty list, and that gaps left by deleted tasks are not reused.

## Completion criteria
- `add` creates a task with the correct next ID and `done: false`, then saves.
- ID assignment starts at 1 for an empty list and never reuses IDs of deleted tasks.
- Unit tests for all three ID assignment scenarios pass.
- `cargo test` passes with zero failures.
- `cargo clippy --all-targets --all-features -- -D warnings` reports zero warnings.

## Commit
dd74a5cbaa7ad60e7c4feaf592a7d2d6eeee8c8c
