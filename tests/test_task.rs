#[cfg(test)]
mod tests {
    use std::fs;
    use tasky_cli::{Task, load_tasks, save_tasks};

    #[test]
    fn test_add_and_list() {
        let _ = fs::remove_file("tasks.json");

        let mut tasks = vec![Task {
            text: "Test task".to_string(),
            done: false,
        }];
        save_tasks(&tasks);

        let loaded = load_tasks();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].text, "Test task");
        assert!(!loaded[0].done);
    }
}
