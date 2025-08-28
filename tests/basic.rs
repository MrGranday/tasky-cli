#[cfg(test)]
mod tests {
    use serde_json;
    use std::fs;
    use tasky::{Task, save_tasks};

    #[test]
    fn test_add_and_list() {
        // Clear tasks.json for a clean test
        let _ = fs::remove_file("tasks.json");

        // Simulate adding a task
        let mut tasks = vec![];
        tasks.push(Task {
            text: "Test task".to_string(),
            done: false,
        });
        save_tasks(&tasks);

        // Check tasks.json
        let content = fs::read_to_string("tasks.json").expect("Failed to read tasks.json");
        let loaded: Vec<Task> = serde_json::from_str(&content).expect("Failed to parse JSON");
        assert_eq!(loaded.len(), 1, "Expected exactly one task");
        assert_eq!(loaded[0].text, "Test task", "Task text mismatch");
        assert_eq!(loaded[0].done, false, "Task done status mismatch");
    }
}
