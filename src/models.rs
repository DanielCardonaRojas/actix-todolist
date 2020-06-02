use serde::{Deserialize, Serialize};
use slog::Logger;

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    pub uuid: String,
    pub title: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TodoItemNew {
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoItemEdit {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

pub struct AppState {
    pub logger: Logger,
}
