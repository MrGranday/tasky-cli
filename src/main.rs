use clap::{Parser, Subcommand};
use colored::control;
use tasky_cli::storage::load_tasks; // Only import load_tasks

mod commands; // Keep the commands module

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
            commands::add_task(&mut tasks, text);
        }
        Commands::List => {
            commands::list_tasks(&tasks);
        }
        Commands::Remove { index } => {
            commands::remove_task(&mut tasks, index);
        }
        Commands::Done { index } => {
            commands::done_task(&mut tasks, index);
        }
    }
}
