[AGENT-MANAGED FILE — DO NOT MODIFY MANUALLY]

# Next task
## Title
Set up clap scaffold and implement `list`

## Description
Wire the clap derive CLI structure: define the top-level `Cli` struct and the `Commands` enum
covering all four subcommands. Implement the `list` subcommand: load tasks from storage, print
each task ordered by ID in the format `<id> [<status>] <title>` (using `[ ]` for pending and
`[x]` for done), and print a human-readable empty-list message if there are no tasks. Add a
unit test for `list` output against an in-memory task list.

## Completion criteria
- Clap CLI scaffold compiles with all four subcommands declared.
- `list` prints tasks in the correct format and exits 0.
- `list` on an empty store prints a human-readable message and exits 0.
- Unit test for list output passes.
- `cargo test` passes with zero failures.
- `cargo clippy --all-targets --all-features -- -D warnings` reports zero warnings.

## Commit
b47d61bb854a60fad630c12541e97f051a0f8195
