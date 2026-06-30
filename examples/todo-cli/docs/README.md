# todo-cli

A simple command-line todo manager that stores tasks as JSON on your local filesystem.

## Installation

Requires Rust stable (see [MSRV](#msrv)).

```
git clone <repo-url>
cd todo-cli
cargo install --path .
```

## Building

```
cargo build --release
```

The binary is written to `target/release/todo-cli`.

## Usage

#### Commands

- Print help menu:
```
$ todo-cli help
A simple command-line todo manager

Usage: todo-cli <COMMAND>

Commands:
  add     Add a new task
  list    List all tasks
  done    Mark a task as done
  delete  Delete a task
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

- List all tasks:
```
$ todo-cli list
No tasks.
```

- Add a new task:
```
$ todo-cli add "Buy milk"
$ todo-cli add "Water plants"
$ todo-cli list
1 [ ] Buy milk
2 [ ] Water plants
```

- Mark a task as done:
```
$ todo-cli done 1
$ todo-cli list
1 [x] Buy milk
2 [ ] Water plants
```

- Delete a task:
```
$ todo-cli delete 1
$ todo-cli list
2 [ ] Water plants
```

#### Storage
Tasks are stored in `~/.todo-cli/tasks.json` by default. Override the path with the
`TODO_CLI_FILE` environment variable:

```
TODO_CLI_FILE=/tmp/tasks.json todo-cli list
```

## Testing

```
cargo test
```

## MSRV

Rust stable. The crate uses the 2024 edition.

## Contributing

Bug reports and pull requests are welcome. Please open an issue before submitting a
large change.

## License

MIT
