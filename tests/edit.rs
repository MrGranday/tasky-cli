#[cfg(test)]
mod tests {
    use std::fs;

    use tasky_cli::{
        Task,
        commands::edit_task,
        storage::{load_tasks_from_file, save_tasks_to_file},
    };

    fn setup_test_environment(file: &str) {
        let _ = fs::remove_file(file); // Clean up before test
        let initial_tasks = vec![
            Task {
                text: "Task 1".to_string(),
                done: false,
                date_string: Some("2024-01-01".to_string()),
            },
            Task {
                text: "Task 2".to_string(),
                done: false,
                date_string: Some("2024-01-02".to_string()),
            },
            Task {
                text: "Task 3".to_string(),
                done: false,
                date_string: Some("2024-01-03".to_string()),
            },
        ];
        save_tasks_to_file(&initial_tasks, file);
    }

    #[test]
    fn test_edit_task_success() {
        let file = "test_edit_success.json";
        setup_test_environment(file);
        let mut tasks = load_tasks_from_file(file);
        let new_text = "Edited Task 2".to_string();
        let index_to_edit = 1;
        edit_task(&mut tasks, index_to_edit, new_text.clone(), file);
        let updated_tasks = load_tasks_from_file(file);
        assert_eq!(updated_tasks.len(), 3, "Task count should not change");
        assert_eq!(
            updated_tasks[index_to_edit].text, new_text,
            "Task text should be updated"
        );
        let _ = fs::remove_file(file); // Clean up after test
    }

    #[test]
    fn test_edit_task_invalid_index() {
        let file = "test_edit_invalid.json";
        setup_test_environment(file);
        let mut tasks = load_tasks_from_file(file);
        let new_text = "This should not be saved".to_string();
        let invalid_index = 99;
        edit_task(&mut tasks, invalid_index, new_text.clone(), file);
        let unchanged_tasks = load_tasks_from_file(file);
        assert_eq!(unchanged_tasks.len(), 3, "Task count should not change");
        assert_ne!(
            unchanged_tasks[0].text, new_text,
            "No task should be updated"
        );
        let _ = fs::remove_file(file); // Clean up after test
    }
}
