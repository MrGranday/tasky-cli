#[cfg(test)]
mod tests {

    use tasky_cli::storage;
    use tasky_cli::{
        Task,
        commands::remove_task,
        storage::{load_tasks, save_tasks},
    };

    #[test]
    fn test_remove_task() {
        storage::set_tasks_file("test_remove.json");
        let _ = std::fs::remove_file("test_remove.json");

        let mut tasks = vec![
            Task {
                text: "Task 1".to_string(),
                done: false,
                date_string: Some("2025-08-03".to_string()),
            },
            Task {
                text: "Task 2".to_string(),
                done: false,
                date_string: Some("2025-08-03".to_string()),
            },
        ];
        save_tasks(&tasks);

        let loaded = load_tasks();
        assert_eq!(loaded.len(), 2, "Expected two tasks initially");
        assert_eq!(loaded[0].text, "Task 1", "Initial Task 1 text mismatch");
        assert_eq!(loaded[1].text, "Task 2", "Initial Task 2 text mismatch");

        remove_task(&mut tasks, 0);

        let loaded = load_tasks();
        assert_eq!(loaded.len(), 1, "Expected one task after removal");
        assert_eq!(loaded[0].text, "Task 2", "Expected Task 2 to remain");
        assert!(!loaded[0].done, "Remaining task should not be done");
    }
}
