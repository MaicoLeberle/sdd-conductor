[AGENT-MANAGED FILE — DO NOT MODIFY MANUALLY]

# Next task
## Title
Implement `delete` with error handling

## Description
Implement the `delete <id>` subcommand: find the task by ID, remove it, and save; if the ID
does not exist, print an informative message to stderr and exit non-zero. Add unit tests for
the success case and the not-found case. Run `cargo audit`.

## Completion criteria
- `delete` removes the correct task and saves.
- `delete` with an unknown ID exits non-zero and writes an informative message to stderr.
- Unit tests for success and not-found cases pass.
- `cargo test` passes with zero failures.
- `cargo clippy --all-targets --all-features -- -D warnings` reports zero warnings.
- `cargo audit` reports no known vulnerabilities.

## Commit
3a11e76
