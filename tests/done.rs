#[cfg(test)]
mod tests {

    use tasky_cli::storage;
    use tasky_cli::{
        Task,
        commands::done_task,
        storage::{load_tasks, save_tasks},
    };
    #[test]
    fn test_done_task() {
        storage::set_tasks_file("test_done.json");
        let _ = std::fs::remove_file("test_done.json");

        // Setup: Save one task
        let mut tasks = vec![Task {
            text: "Test task".to_string(),
            date_string: Some("2025-08-03".to_string()),
            done: false,
        }];
        save_tasks(&tasks);

        // Verify initial state
        let loaded = load_tasks();
        assert_eq!(loaded.len(), 1, "Expected one task initially");
        assert!(!loaded[0].done, "Task should not be done initially");

        // Mark task as done
        done_task(&mut tasks, 0);

        // Verify updated state
        let loaded = load_tasks();
        assert_eq!(loaded.len(), 1, "Expected one task after marking done");
        assert_eq!(loaded[0].text, "Test task", "Task text mismatch");
        assert!(loaded[0].done, "Task should be marked done");
    }
}
