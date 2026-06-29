use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A single to-do item.
#[derive(Serialize, Deserialize)]
pub struct Task {
    /// Unique, monotonically increasing identifier. Never reused after deletion.
    pub id: u64,
    /// Human-readable description of the task.
    pub title: String,
    /// Whether the task has been marked as completed.
    pub done: bool,
}

/// Returns the path to the JSON tasks file.
///
/// If the `TODO_CLI_FILE` environment variable is set its value is used directly;
/// otherwise the default is `~/.todo-cli/tasks.json`.
pub fn resolve_storage_path() -> PathBuf {
    if let Ok(path) = std::env::var("TODO_CLI_FILE") {
        return PathBuf::from(path);
    }
    dirs::home_dir()
        .expect("could not determine home directory")
        .join(".todo-cli")
        .join("tasks.json")
}

/// Loads tasks from `path`.
///
/// Returns an empty list if the file does not exist yet.
pub fn load_tasks(path: &PathBuf) -> Result<Vec<Task>, std::io::Error> {
    match fs::read_to_string(path) {
        Ok(content) => {
            let tasks = serde_json::from_str(&content)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            Ok(tasks)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(vec![]),
        Err(e) => Err(e),
    }
}

/// Serialises `tasks` to pretty-printed JSON and writes them to `path`.
///
/// Parent directories are created automatically if they do not exist.
pub fn save_tasks(path: &PathBuf, tasks: &[Task]) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(tasks)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    fs::write(path, json)
}

/// Returns the next available task ID (`max(existing ids) + 1`, or `1` for an empty list).
pub fn next_id(tasks: &[Task]) -> u64 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

/// Removes the task with the given `id` from `tasks`.
///
/// Returns `true` if a task was removed, `false` if no task with that ID existed.
pub fn delete_task(tasks: &mut Vec<Task>, id: u64) -> bool {
    let len_before = tasks.len();
    tasks.retain(|t| t.id != id);
    tasks.len() < len_before
}

/// Marks the task with the given `id` as done.
///
/// Returns `true` if the task was found and updated, `false` if no task with that ID existed.
pub fn mark_done(tasks: &mut [Task], id: u64) -> bool {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.done = true;
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_task_serialization_round_trip_single() {
        let task = Task { id: 1, title: "Buy milk".to_string(), done: false };
        let json = serde_json::to_string(&task).unwrap();
        let restored: Task = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, 1);
        assert_eq!(restored.title, "Buy milk");
        assert!(!restored.done);
    }

    #[test]
    fn test_task_serialization_round_trip_list() {
        let tasks = vec![
            Task { id: 1, title: "Buy milk".to_string(), done: false },
            Task { id: 2, title: "Walk dog".to_string(), done: true },
        ];
        let json = serde_json::to_string(&tasks).unwrap();
        let restored: Vec<Task> = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.len(), 2);
        assert_eq!(restored[0].id, 1);
        assert_eq!(restored[1].title, "Walk dog");
        assert!(restored[1].done);
    }

    #[test]
    fn test_load_tasks_missing_file_returns_empty() {
        let path = PathBuf::from("/tmp/todo_cli_test_nonexistent_file_xyz.json");
        let tasks = load_tasks(&path).unwrap();
        assert!(tasks.is_empty());
    }

    #[test]
    fn test_save_and_load_round_trip() {
        let dir = tempfile_path();
        let path = dir.join("tasks.json");
        let tasks = vec![
            Task { id: 1, title: "First".to_string(), done: false },
            Task { id: 2, title: "Second".to_string(), done: true },
        ];
        save_tasks(&path, &tasks).unwrap();
        let loaded = load_tasks(&path).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].id, 1);
        assert_eq!(loaded[0].title, "First");
        assert!(!loaded[0].done);
        assert_eq!(loaded[1].id, 2);
        assert!(loaded[1].done);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_resolve_storage_path_uses_env_var() {
        let expected = "/tmp/custom_todo.json";
        // SAFETY: single-threaded test binary; no concurrent env access.
        unsafe { env::set_var("TODO_CLI_FILE", expected) };
        let path = resolve_storage_path();
        unsafe { env::remove_var("TODO_CLI_FILE") };
        assert_eq!(path, PathBuf::from(expected));
    }

    #[test]
    fn test_delete_task_success() {
        let mut tasks = vec![
            Task { id: 1, title: "A".to_string(), done: false },
            Task { id: 2, title: "B".to_string(), done: false },
        ];
        assert!(delete_task(&mut tasks, 1));
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, 2);
    }

    #[test]
    fn test_delete_task_not_found() {
        let mut tasks = vec![Task { id: 1, title: "A".to_string(), done: false }];
        assert!(!delete_task(&mut tasks, 99));
        assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn test_mark_done_success() {
        let mut tasks = vec![
            Task { id: 1, title: "A".to_string(), done: false },
            Task { id: 2, title: "B".to_string(), done: false },
        ];
        assert!(mark_done(&mut tasks, 1));
        assert!(tasks[0].done);
        assert!(!tasks[1].done);
    }

    #[test]
    fn test_mark_done_not_found() {
        let mut tasks = vec![Task { id: 1, title: "A".to_string(), done: false }];
        assert!(!mark_done(&mut tasks, 99));
        assert!(!tasks[0].done);
    }

    #[test]
    fn test_next_id_empty_list() {
        assert_eq!(next_id(&[]), 1);
    }

    #[test]
    fn test_next_id_non_empty_list() {
        let tasks = vec![
            Task { id: 1, title: "A".to_string(), done: false },
            Task { id: 2, title: "B".to_string(), done: true },
        ];
        assert_eq!(next_id(&tasks), 3);
    }

    #[test]
    fn test_next_id_never_reuses_deleted_gap() {
        // IDs 1 and 3 exist; 2 was deleted. Next ID must be 4, not 2.
        let tasks = vec![
            Task { id: 1, title: "A".to_string(), done: false },
            Task { id: 3, title: "C".to_string(), done: false },
        ];
        assert_eq!(next_id(&tasks), 4);
    }

    fn tempfile_path() -> PathBuf {
        use std::time::{SystemTime, UNIX_EPOCH};
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
        PathBuf::from(format!("/tmp/todo_cli_test_{}", ts))
    }
}
