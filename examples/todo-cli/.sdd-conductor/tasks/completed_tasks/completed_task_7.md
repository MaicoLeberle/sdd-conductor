[AGENT-MANAGED FILE — DO NOT MODIFY MANUALLY]

# Next task
## Title
Implement `done` with error handling

## Description
Implement the `done <id>` subcommand: find the task by ID, mark it as done, and save; if the
ID does not exist, print an informative message to stderr and exit non-zero. Initialize logging
with `env_logger::init()` at the top of `main()`. Add unit tests for the success case and the
not-found case.

## Completion criteria
- `done` marks the correct task as done and saves.
- `done` with an unknown ID exits non-zero and writes an informative message to stderr.
- `env_logger` is initialized in `main()`.
- Unit tests for success and not-found cases pass.
- `cargo test` passes with zero failures.
- `cargo clippy --all-targets --all-features -- -D warnings` reports zero warnings.

## Commit
37b0b111fde81447e827807802d04eeb771c4338
