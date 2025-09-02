use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub text: String,
    pub done: bool,
    pub date_string: String,
}

impl Task {
    pub fn new(text: String, date_string: String) -> Self {
        Self {
            text,
            done: false,
            date_string,
        }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }
}
