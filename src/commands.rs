use crate::storage::save_tasks_to_file;
use crate::task::Task;
use chrono::DateTime;
use chrono::offset::Utc;
use colored::*;

pub fn add_task(tasks: &mut Vec<Task>, text: String, date_string: Option<String>, file: &str) {
    let date = date_string.unwrap_or_else(|| "".to_string()); // empty if none provided
    let new_task = Task::new(text.clone(), date.clone());
    tasks.push(new_task.clone());
    save_tasks_to_file(tasks, file);

    if date.is_empty() {
        println!("Added: {}", text.green());
        println!("Tip: You can also add a due date later using:");
        println!("   tasky-cli edit <index> \"<new_text>\"  OR");
        println!("   tasky-cli add \"task\" \"YYYY-MM-DD\"");
    } else {
        println!("Added: {} --due {}", text.green(), date.blue());
    }
}

pub fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks yet!");
    } else {
        let now: DateTime<Utc> = Utc::now();
        let current_date = now.format("%Y-%m-%d").to_string();

        for (i, task) in tasks.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[ ]" };

            if let Some(date) = &task.date_string {
                let (task_text, date_text) = if current_date > *date {
                    (task.text.blue(), date.red())
                } else {
                    (task.text.blue(), date.yellow())
                };
                println!("{}: {} {} due {}", i, status, task_text, date_text);
            } else {
                println!("{}: {} {}", i, status, task.text.blue());
            }
        }
    }
}

pub fn remove_task(tasks: &mut Vec<Task>, index: usize, file: &str) {
    if index < tasks.len() {
        let task = tasks.remove(index);
        save_tasks_to_file(tasks, file);
        println!("Removed: {}", task.text.red());
    } else {
        println!("Error: Invalid index {}", index);
    }
}

pub fn done_task(tasks: &mut Vec<Task>, index: usize, file: &str) {
    if index < tasks.len() {
        tasks[index].mark_done();
        save_tasks_to_file(tasks, file);
        println!("Marked as done: {}", tasks[index].text.green());
    } else {
        println!("Error: Invalid index {}", index);
    }
}

pub fn edit_task(tasks: &mut Vec<Task>, index: usize, new_text: String, file: &str) {
    if index < tasks.len() {
        let old_text = tasks[index].text.clone();
        tasks[index].text = new_text.clone();
        save_tasks_to_file(tasks, file);
        println!(
            "Edited task {}: '{}' -> '{}'",
            index,
            old_text.yellow(),
            new_text.green()
        );
    } else {
        println!("Error: Invalid index {index}");
    }
}
