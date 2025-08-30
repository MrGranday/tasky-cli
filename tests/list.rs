#[cfg(test)]
mod tests {
    use std::fs;
    use tasky_cli::{
        Task,
        commands::list_tasks,
        storage::{load_tasks, save_tasks},
    };

    #[test]
    fn test_list_tasks() {
        let _ = fs::remove_file("tasks.json");

        // Setup: Save some tasks
        let tasks = vec![
            Task {
                text: "Task 1".to_string(),
                done: false,
            },
            Task {
                text: "Task 2".to_string(),
                done: true,
            },
        ];
        save_tasks(&tasks);

        // List doesn't modify tasks, so we just verify tasks are unchanged
        let before = load_tasks();
        list_tasks(&before);
        let after = load_tasks();
        assert_eq!(before.len(), after.len(), "List should not modify tasks");
        assert_eq!(before[0].text, after[0].text, "Task 1 text mismatch");
        assert_eq!(before[1].text, after[1].text, "Task 2 text mismatch");
        assert_eq!(before[0].done, after[0].done, "Task 1 done status mismatch");
        assert_eq!(before[1].done, after[1].done, "Task 2 done status mismatch");
    }
}
