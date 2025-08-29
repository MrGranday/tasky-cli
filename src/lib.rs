use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub text: String,
    pub done: bool,
}

pub fn load_tasks() -> Vec<Task> {
    fs::read_to_string("tasks.json")
        .map(|s| serde_json::from_str(&s).unwrap_or(vec![]))
        .unwrap_or(vec![])
}

pub fn save_tasks(tasks: &[Task]) {
    fs::write("tasks.json", serde_json::to_string_pretty(tasks).unwrap()).unwrap();
}
