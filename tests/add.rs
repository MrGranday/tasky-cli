#[cfg(test)]
mod tests {
    use std::fs;
    use tasky_cli::{Task, commands::add_task, storage::load_tasks};

    #[test]
    fn test_add_task() {
        let _ = fs::remove_file("tasks.json");

        let mut tasks = vec![];
        add_task(&mut tasks, "Test task".to_string());

        let loaded = load_tasks();
        assert_eq!(loaded.len(), 1, "Expected exactly one task");
        assert_eq!(loaded[0].text, "Test task", "Task text mismatch");
        assert!(!loaded[0].done, "Task should not be done");
    }
}
