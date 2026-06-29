<mark>THIS FILE CONTAINS THE DESCRIPTION OF THE DESIRED PROJECT, AS SET OF GOALS, NON-GOALS,
CONSTRAINTS, AND SUCCESS CRITERIA.</mark>


## *GOALS*

Implement a command-line todo list manager called `todo-cli` that persists tasks to a local JSON
file. The tool must support the following operations:

- `todo-cli add "<title>"` — create a new task with the given title and assign it the next
  available integer ID. The task is initially not done.
- `todo-cli list` — print all tasks ordered by ID. Each line shows the task ID, a status
  indicator (`[ ]` for pending, `[x]` for done), and the title.
- `todo-cli done <id>` — mark the task with the given ID as done. Exit with a non-zero code and
  an informative error message if the ID does not exist.
- `todo-cli delete <id>` — remove the task with the given ID. Exit with a non-zero code and an
  informative error message if the ID does not exist.

State must persist across invocations. The JSON storage file is read at startup and written after
every mutating command. The default storage path is `~/.todo-cli/tasks.json`; it can be overridden
by setting the `TODO_CLI_FILE` environment variable to an alternative absolute path. The parent
directory is created automatically on first use if it does not exist.


## *NON-GOALS*

- No subtasks, priorities, due dates, or tags.
- No interactive or TUI mode.
- No remote sync, multi-user support, or networked storage.
- No import or export formats other than the internal JSON store.
- No undo or history of deleted tasks.
- No search or filter functionality beyond what `list` already provides.
- No colour or terminal-formatting output beyond plain ASCII status indicators.


## *CONSTRAINTS*

- Language: Rust (stable toolchain).
- CLI argument parsing: `clap` (derive API).
- Serialization: `serde` and `serde_json`.
- No database; storage is a single flat JSON file containing an array of task objects.
- Each task object has three fields: `id` (u64), `title` (String), `done` (bool).
- IDs are assigned as `max(existing ids) + 1`, starting from 1 for an empty list. IDs of deleted
  tasks are never reused.
- All user-visible output goes to stdout. Error messages (unknown ID, I/O failures) go to stderr.
- The tool must compile and pass `cargo clippy --all-targets --all-features -- -D warnings` with
  zero warnings.
- Minimal dependencies: only `clap`, `serde`, `serde_json`, and `dirs` (for resolving `~`) are
  permitted. No other third-party crates.


## *SUCCESS CRITERIA*

- `todo-cli add "Buy milk"` exits 0, creates or updates the storage file, and the task appears
  in subsequent `list` output with status `[ ]`.
- `todo-cli list` on an empty store prints a human-readable empty-list message and exits 0.
- `todo-cli done 1` exits 0 and causes the subsequent `list` to show the task with status `[x]`.
- `todo-cli delete 1` exits 0 and causes the subsequent `list` to no longer show the task.
- `todo-cli done 99` (non-existent ID) exits non-zero and prints an informative message to stderr.
- `todo-cli delete 99` (non-existent ID) exits non-zero and prints an informative message to
  stderr.
- IDs of deleted tasks are never reused in subsequent `add` calls.
- Running `todo-cli list` in a fresh shell after prior mutations confirms state persists across
  invocations.
- `TODO_CLI_FILE=/tmp/test.json todo-cli add "x"` writes to `/tmp/test.json` and not to the
  default path.
- `cargo test` passes with at minimum unit tests for: ID assignment logic, add/list/done/delete
  operations against an in-memory task list, and storage round-trip serialization.
