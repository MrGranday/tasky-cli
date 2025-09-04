use clap::{Parser, Subcommand};
use colored::control;
use tasky_cli::commands::{add_task, done_task, edit_task, list_tasks, remove_task};
use tasky_cli::storage::{load_tasks, tasks_file};

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
    ///add a task
    Add {
        text: String,
        date_string: Option<String>,
    },

    ///list tasks
    List,

    ///edit tasks
    Edit { index: usize, new_text: String },

    ///remove task
    Remove { index: usize },

    /// mark task as done
    Done { index: usize },
}

fn main() {
    control::set_override(true);

    let args = Args::parse();
    let mut tasks = load_tasks();
    let file = tasks_file(); // Get default file path

    match args.command {
        Commands::Add { text, date_string } => {
            add_task(&mut tasks, text, date_string, &file);
        }
        Commands::List => {
            list_tasks(&tasks);
        }
        Commands::Remove { index } => {
            remove_task(&mut tasks, index, &file);
        }
        Commands::Done { index } => {
            done_task(&mut tasks, index, &file);
        }
        Commands::Edit { index, new_text } => {
            edit_task(&mut tasks, index, new_text, &file);
        }
    }
}
