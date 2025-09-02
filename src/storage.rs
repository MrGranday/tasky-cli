use serde_json;
use std::fs;

use crate::task::Task;

const TASKS_FILE: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    fs::read_to_string(TASKS_FILE)
        .map(|s| serde_json::from_str(&s).unwrap_or(vec![]))
        .unwrap_or(vec![])
}

pub fn save_tasks(tasks: &[Task]) {
    println!("{}", tasks.len());
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write(TASKS_FILE, json).expect("Failed to write tasks file");
}
