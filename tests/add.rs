#[cfg(test)]
mod tests {
    use std::fs;
    use tasky_cli::storage;
    use tasky_cli::{commands::add_task, storage::load_tasks};

    #[test]
    fn test_add_task_with_date() {
        storage::set_tasks_file("test_add_with_date.json");
        let _ = std::fs::remove_file("test_add_with_date.json"); // cleanup

        let mut tasks = vec![];
        add_task(
            &mut tasks,
            "Test task".to_string(),
            Some("2025-08-03".to_string()),
        );

        let loaded = load_tasks();
        assert_eq!(loaded.len(), 1, "Expected exactly one task");
        assert_eq!(loaded[0].text, "Test task", "Task text mismatch");
        assert!(!loaded[0].done, "Task should not be done");
        assert_eq!(loaded[0].date_string, Some("2025-08-03".to_string()));
    }

    #[test]
    fn test_add_task_without_date() {
        let _ = fs::remove_file("tasks.json");

        let mut tasks = vec![];
        add_task(&mut tasks, "Task without date".to_string(), None);

        let loaded = load_tasks();
        assert_eq!(loaded.len(), 1, "Expected one task");
        assert_eq!(loaded[0].text, "Task without date");
        assert!(loaded[0].date_string.is_none(), "Expected no due date");
    }
}
