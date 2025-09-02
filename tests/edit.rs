#[cfg(test)]
mod tests {
    use std::fs;
use tasky_cli::{
        Task,
        commands::{done_task,edit_task},
        storage::{load_tasks, save_tasks},
    };

    fn setup_test_environment() {
        let _ = fs::remove_file("tasks.json");
        let initial_tasks = vec![
            Task { text: "Task 1".to_string(), done: false, date_string: "2024-01-01".to_string() },
            Task { text: "Task 2".to_string(), done: false, date_string: "2024-01-02".to_string() },
            Task { text: "Task 3".to_string(), done: false, date_string: "2024-01-03".to_string() },
        ];
        save_tasks(&initial_tasks);
    }

    #[test]
    fn test_edit_task_success() {
        setup_test_environment();
        let mut tasks = load_tasks();
        let new_text = "Edited Task 2".to_string();
        let index_to_edit = 1;
        let result = edit_task(&mut tasks, index_to_edit, new_text.clone());
        assert!(result.is_ok());
        let updated_tasks = load_tasks();
        assert_eq!(updated_tasks.len(), 3, "Task count should not change");
        assert_eq!(updated_tasks[index_to_edit].text, new_text, "Task text should be updated");
    }

    #[test]
    fn test_edit_task_invalid_index() {
        setup_test_environment();

        let mut tasks = load_tasks();
        let new_text = "This should not be saved".to_string();
        let invalid_index = 99;
        let result = edit_task(&mut tasks, invalid_index, new_text.clone());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), format!("Invalid index, not tasks with index {}", invalid_index));
        let unchanged_tasks = load_tasks();
        assert_eq!(unchanged_tasks.len(), 3, "Task count should not change");
        assert_ne!(unchanged_tasks[0].text, new_text, "No task should be updated");
    }
}