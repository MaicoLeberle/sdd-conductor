[AGENT-MANAGED FILE — DO NOT MODIFY MANUALLY]

## *ARCHITECTURE*
`todo-cli` is a single-binary Rust CLI application with two source modules:

- **`src/tasks.rs`** — data model and storage layer. Owns the `Task` struct and all file I/O.
- **`src/main.rs`** — CLI entry point. Parses arguments via clap and dispatches to the
  appropriate logic, using items from `tasks`.

There is no network layer, database, or async runtime. All state is persisted to a single
JSON file on disk.
