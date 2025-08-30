use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub text: String,
    pub done: bool,
}

impl Task {
    pub fn new(text: String) -> Self {
        Self { text, done: false }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }
}
