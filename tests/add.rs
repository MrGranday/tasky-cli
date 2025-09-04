#[cfg(test)]
mod tests {
    use std::fs;
    use tasky_cli::{commands::add_task, storage::load_tasks_from_file};

    #[test]
    fn test_add_task_with_date() {
        let file = "test_add_with_date.json";
        let _ = fs::remove_file(file); // Clean up before test

        let mut tasks = vec![];
        add_task(
            &mut tasks,
            "Test task".to_string(),
            Some("2025-08-03".to_string()),
            file,
        );

        let loaded = load_tasks_from_file(file);
        assert_eq!(loaded.len(), 1, "Expected exactly one task");
        assert_eq!(loaded[0].text, "Test task", "Task text mismatch");
        assert!(!loaded[0].done, "Task should not be done");
        assert_eq!(loaded[0].date_string, Some("2025-08-03".to_string()));

        let _ = fs::remove_file(file); // Clean up after test
    }

    #[test]
    fn test_add_task_without_date() {
        let file = "test_add_without_date.json";
        let _ = fs::remove_file(file); // Clean up before test

        let mut tasks = vec![];
        add_task(&mut tasks, "Task without date".to_string(), None, file);

        let loaded = load_tasks_from_file(file);
        assert_eq!(loaded.len(), 1, "Expected one task");
        assert_eq!(loaded[0].text, "Task without date", "Task text mismatch");
        assert!(loaded[0].date_string.is_none(), "Expected no due date");

        let _ = fs::remove_file(file); // Clean up after test
    }
}
