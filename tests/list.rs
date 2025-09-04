#[cfg(test)]
mod tests {
    use std::fs;

    use tasky_cli::{
        Task,
        commands::list_tasks,
        storage::{load_tasks_from_file, save_tasks_to_file},
    };

    #[test]
    fn test_list_tasks() {
        let file = "test_list.json";
        let _ = fs::remove_file(file); // Clean up before test

        // Setup: Save some tasks
        let tasks = vec![
            Task {
                text: "Task 1".to_string(),
                done: false,
                date_string: Some("2025-08-03".to_string()),
            },
            Task {
                text: "Task 2".to_string(),
                done: true,
                date_string: Some("2025-08-03".to_string()),
            },
            Task {
                text: "Task 3 (no date)".to_string(),
                done: false,
                date_string: None,
            },
        ];
        save_tasks_to_file(&tasks, file);

        // List doesn't modify tasks
        let before = load_tasks_from_file(file);
        list_tasks(&before);
        let after = load_tasks_from_file(file);

        assert_eq!(before.len(), after.len(), "List should not modify tasks");
        assert_eq!(before[0].text, after[0].text, "Task 1 text mismatch");
        assert_eq!(before[1].text, after[1].text, "Task 2 text mismatch");
        assert_eq!(before[2].text, after[2].text, "Task 3 text mismatch");

        let _ = fs::remove_file(file); // Clean up after test
    }
}
