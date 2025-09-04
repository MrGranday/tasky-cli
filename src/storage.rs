use serde_json;
use std::fs;
use std::sync::OnceLock;

use crate::task::Task;

// Global storage for overriding file path (used in tests)
static TASKS_FILE: OnceLock<String> = OnceLock::new();

/// Set a custom file path (mainly for tests)
pub fn set_tasks_file(path: &str) {
    // Ignore error if already set (so first call wins)
    let _ = TASKS_FILE.set(path.to_string());
}

/// Get current tasks file, defaults to "tasks.json"
fn tasks_file() -> String {
    TASKS_FILE
        .get()
        .cloned()
        .unwrap_or_else(|| "tasks.json".to_string())
}

pub fn load_tasks() -> Vec<Task> {
    fs::read_to_string(tasks_file())
        .map(|s| serde_json::from_str(&s).unwrap_or(vec![]))
        .unwrap_or(vec![])
}

pub fn save_tasks(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write(tasks_file(), json).expect("Failed to write tasks file");
}
