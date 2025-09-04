use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub text: String,
    pub done: bool,
    pub date_string: Option<String>, // <-- optional
}

impl Task {
    pub fn new(text: String, date_string: String) -> Self {
        let date_opt = if date_string.trim().is_empty() {
            None
        } else {
            Some(date_string)
        };
        Self {
            text,
            done: false,
            date_string: date_opt,
        }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }
}
