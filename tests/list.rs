#[cfg(test)]
mod tests {

    use tasky_cli::storage;
    use tasky_cli::{
        Task,
        commands::list_tasks,
        storage::{load_tasks, save_tasks},
    };

    #[test]
    fn test_list_tasks() {
        storage::set_tasks_file("test_list.json");
        let _ = std::fs::remove_file("test_list.json");

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
        save_tasks(&tasks);

        // List doesn't modify tasks
        let before = load_tasks();
        list_tasks(&before);
        let after = load_tasks();

        assert_eq!(before.len(), after.len(), "List should not modify tasks");
        assert_eq!(before[0].text, after[0].text, "Task 1 text mismatch");
        assert_eq!(before[1].text, after[1].text, "Task 2 text mismatch");
        assert_eq!(before[2].text, after[2].text, "Task 3 text mismatch");
    }
}
