use clap::{Parser, Subcommand};
use colored::control;
use tasky_cli::commands::{add_task, done_task, edit_task, list_tasks, remove_task};
use tasky_cli::storage::load_tasks; // Import from tasky_cli

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
    Add {
        text: String,
        date_string: Option<String>,
    },
    List,
    Edit {
        index: usize,
        new_text: String,
    },
    Remove {
        index: usize,
    },
    Done {
        index: usize,
    },
}

fn main() {
    control::set_override(true);

    let args = Args::parse();
    let mut tasks = load_tasks();

    match args.command {
        Commands::Add { text, date_string } => {
            add_task(&mut tasks, text, date_string);
        }
        Commands::List => {
            list_tasks(&tasks);
        }
        Commands::Remove { index } => {
            remove_task(&mut tasks, index);
        }
        Commands::Done { index } => {
            done_task(&mut tasks, index);
        }
        Commands::Edit { index, new_text } => {
            edit_task(&mut tasks, index, new_text);
        }
    }
}
