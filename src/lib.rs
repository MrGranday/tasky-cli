pub mod storage;
pub mod task;

// Re-export for convenience
pub use storage::{load_tasks, save_tasks};
pub use task::Task;
