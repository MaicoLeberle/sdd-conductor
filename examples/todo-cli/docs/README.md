# todo-cli

A simple command-line todo manager that stores tasks as JSON on your local filesystem.

## Installation

Requires Rust stable (see [MSRV](#msrv)).

```sh
git clone <repo-url>
cd todo-cli
cargo install --path .
```

## Building

```sh
cargo build --release
```

The binary is written to `target/release/todo-cli`.

## Usage

```sh
# Add a new task
todo-cli add "Buy milk"

# List all tasks
todo-cli list

# Mark a task as done (replace 1 with the task ID)
todo-cli done 1

# Delete a task
todo-cli delete 1
```

Tasks are stored in `~/.todo-cli/tasks.json` by default. Override the path with the
`TODO_CLI_FILE` environment variable:

```sh
TODO_CLI_FILE=/tmp/tasks.json todo-cli list
```

## Testing

```sh
cargo test
```

## MSRV

Rust stable. The crate uses the 2024 edition.

## Contributing

Bug reports and pull requests are welcome. Please open an issue before submitting a
large change.

## License

MIT
