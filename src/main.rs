use tasky_cli::{Task, load_tasks, save_tasks};

use clap::{Parser, Subcommand};
use colored::control;
use colored::*;

#[derive(Parser)]
#[command(
    name = "tasky-cli",
    about = "A simple to-do list CLI",
    version = env!("CARGO_PKG_VERSION")
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { text: String },
    List,
    Remove { index: usize },
    Done { index: usize },
}

fn main() {
    control::set_override(true);

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
        Commands::Done { index } => {
            if index < tasks.len() {
                tasks[index].done = true;
                save_tasks(&tasks);
                println!("Marked as done: {}", tasks[index].text.green());
            } else {
                println!("Error: Invalid index {}", index);
            }
        }
    }
}
