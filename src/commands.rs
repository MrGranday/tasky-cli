use colored::*;
use tasky_cli::storage::save_tasks;
use tasky_cli::task::Task;

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
