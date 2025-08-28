use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Task {
    text: String,
    done: bool,
}

#[derive(Parser)]
#[command(name = "tasky", about = "A simple to-do list CLI")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { text: String },
    /// List all tasks
    List,
    /// Remove a task by index
    Remove { index: usize },
}

fn load_tasks() -> Vec<Task> {
    fs::read_to_string("tasks.json")
        .map(|s| serde_json::from_str(&s).unwrap_or(vec![]))
        .unwrap_or(vec![])
}

pub fn save_tasks(tasks: &[Task]) {
    fs::write("tasks.json", serde_json::to_string_pretty(tasks).unwrap()).unwrap();
}

fn main() {
    let args = Args::parse();
    let mut tasks = load_tasks();

    match args.command {
        Commands::Add { text } => {
            tasks.push(Task {
                text: text.clone(),
                done: false,
            });
            save_tasks(&tasks);
            println!("Added: {}", text.green());
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks yet!");
            } else {
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.done { "[X]" } else { "[ ]" };
                    println!("{}: {} {}", i, status.yellow(), task.text.blue());
                }
            }
        }
        Commands::Remove { index } => {
            if index < tasks.len() {
                let task = tasks.remove(index);
                save_tasks(&tasks);
                println!("Removed: {}", task.text.red());
            } else {
                println!("Error: Invalid index {}", index);
            }
        }
    }
}
