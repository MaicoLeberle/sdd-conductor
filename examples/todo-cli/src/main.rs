mod tasks;

use clap::{Parser, Subcommand};
use tasks::{Task, delete_task, load_tasks, mark_done, next_id, resolve_storage_path, save_tasks};

#[derive(Parser)]
#[command(about = "A simple command-line todo manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { title: String },
    /// List all tasks
    List,
    /// Mark a task as done
    Done { id: u64 },
    /// Delete a task
    Delete { id: u64 },
}

fn format_task_list(tasks: &[Task]) -> String {
    if tasks.is_empty() {
        return "No tasks.".to_string();
    }
    tasks
        .iter()
        .map(|t| format!("{} [{}] {}", t.id, if t.done { "x" } else { " " }, t.title))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    let path = resolve_storage_path();

    match cli.command {
        Commands::List => {
            let tasks = load_tasks(&path).unwrap_or_else(|e| {
                eprintln!("Error reading tasks: {e}");
                std::process::exit(1);
            });
            println!("{}", format_task_list(&tasks));
        }
        Commands::Add { title } => {
            let mut tasks = load_tasks(&path).unwrap_or_else(|e| {
                eprintln!("Error reading tasks: {e}");
                std::process::exit(1);
            });
            let id = next_id(&tasks);
            tasks.push(Task { id, title, done: false });
            save_tasks(&path, &tasks).unwrap_or_else(|e| {
                eprintln!("Error saving tasks: {e}");
                std::process::exit(1);
            });
        }
        Commands::Done { id } => {
            let mut tasks = load_tasks(&path).unwrap_or_else(|e| {
                eprintln!("Error reading tasks: {e}");
                std::process::exit(1);
            });
            if !mark_done(&mut tasks, id) {
                eprintln!("Error: no task with ID {id}");
                std::process::exit(1);
            }
            save_tasks(&path, &tasks).unwrap_or_else(|e| {
                eprintln!("Error saving tasks: {e}");
                std::process::exit(1);
            });
        }
        Commands::Delete { id } => {
            let mut tasks = load_tasks(&path).unwrap_or_else(|e| {
                eprintln!("Error reading tasks: {e}");
                std::process::exit(1);
            });
            if !delete_task(&mut tasks, id) {
                eprintln!("Error: no task with ID {id}");
                std::process::exit(1);
            }
            save_tasks(&path, &tasks).unwrap_or_else(|e| {
                eprintln!("Error saving tasks: {e}");
                std::process::exit(1);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tasks::Task;

    #[test]
    fn test_format_task_list_empty() {
        assert_eq!(format_task_list(&[]), "No tasks.");
    }

    #[test]
    fn test_format_task_list_pending_and_done() {
        let tasks = vec![
            Task { id: 1, title: "Buy milk".to_string(), done: false },
            Task { id: 2, title: "Walk dog".to_string(), done: true },
        ];
        let output = format_task_list(&tasks);
        assert_eq!(output, "1 [ ] Buy milk\n2 [x] Walk dog");
    }
}
