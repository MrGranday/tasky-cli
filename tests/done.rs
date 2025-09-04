#[cfg(test)]
mod tests {
    use std::fs;
    use tasky_cli::{
        Task,
        commands::done_task,
        storage::{load_tasks_from_file, save_tasks_to_file},
    };

    #[test]
    fn test_done_task() {
        let file = "test_done.json";
        let _ = fs::remove_file(file); // Clean up before test

        // Setup: Save one task
        let mut tasks = vec![Task {
            text: "Test task".to_string(),
            date_string: Some("2025-08-03".to_string()),
            done: false,
        }];
        save_tasks_to_file(&tasks, file);

        // Verify initial state
        let loaded = load_tasks_from_file(file);
        assert_eq!(loaded.len(), 1, "Expected one task initially");
        assert!(!loaded[0].done, "Task should not be done initially");

        // Mark task as done
        done_task(&mut tasks, 0, file);

        // Verify updated state
        let loaded = load_tasks_from_file(file);
        assert_eq!(loaded.len(), 1, "Expected one task after marking done");
        assert_eq!(loaded[0].text, "Test task", "Task text mismatch");
        assert!(loaded[0].done, "Task should be marked done");

        let _ = fs::remove_file(file); // Clean up after test
    }
}
