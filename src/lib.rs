pub mod commands;
pub mod storage;
pub mod task; // Add commands module

// Re-export for convenience
pub use commands::{add_task, done_task, list_tasks, remove_task};
pub use storage::{load_tasks, save_tasks};
pub use task::Task;
