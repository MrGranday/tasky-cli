use crate::storage::save_tasks;
use crate::task::Task;
use colored::*;

pub fn add_task(tasks: &mut Vec<Task>, text: String) {
    let new_task = Task::new(text.clone());
    tasks.push(new_task);
    save_tasks(tasks);
    println!("Added: {}", text.green());
}

pub fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks yet!");
    } else {
        for (i, task) in tasks.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[ ]" };
            println!("{}: {} {}", i, status.yellow(), task.text.blue());
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
        println!("Error: Invalid index {}", index);
    }
}
