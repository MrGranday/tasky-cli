use crate::storage::save_tasks;
use crate::task::Task;
use chrono::DateTime;
use chrono::offset::Utc;
use colored::*;

pub fn add_task(tasks: &mut Vec<Task>, text: String, date_string: String) {
    let new_task = Task::new(text.clone(), date_string.clone());
    println!("Adding task: {}", new_task.date_string);
    tasks.push(new_task.clone());
    save_tasks(tasks);
    println!("Added: {} --due {}", text.green(), date_string.blue());
}

pub fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks yet!");
    } else {
        let now: DateTime<Utc> = Utc::now();
        let current_date = now.format("%Y-%m-%d").to_string();

        for (i, task) in tasks.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[ ]" };
            let (task_text, date_text) = if current_date > task.date_string {
                // Overdue
                (task.text.blue(), task.date_string.red())
            } else {
                // Due today or in the future
                (task.text.blue(), task.date_string.yellow())
            };
            println!("{}: {} {} due {}", i, status, task_text, date_text);
        }
    }
}

pub fn remove_task(tasks: &mut Vec<Task>, index: usize) {
    if index < tasks.len() {
        let task = tasks.remove(index);
        save_tasks(tasks);
        println!("Removed: {}", task.text.red());
    } else {
        println!("Error: Invalid index {}", index);
    }
}

pub fn done_task(tasks: &mut Vec<Task>, index: usize) {
    if index < tasks.len() {
        tasks[index].mark_done();
        save_tasks(tasks);
        println!("Marked as done: {}", tasks[index].text.green());
    } else {
        println!("Error: Invalid index {}", index);
    }
}
pub fn edit_task(tasks: &mut Vec<Task>, index: usize, new_text: String) {
    if index < tasks.len() {
        let old_text = tasks[index].text.clone();
        tasks[index].text = new_text.clone();
        save_tasks(tasks);
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
