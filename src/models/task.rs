use crate::schema::tasks;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[table_name = "tasks"]
#[derive(juniper::GraphQLObject, Serialize, Deserialize, Queryable, Insertable)]
pub struct TodoItem {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(title: String) -> Self {
        TodoItem {
            id: Uuid::new_v4().to_string(),
            title: title,
            completed: false,
        }
    }
}

impl From<TodoItemNew> for TodoItem {
    fn from(item: TodoItemNew) -> Self {
        TodoItem::new(item.title)
    }
}

#[derive(juniper::GraphQLInputObject, Serialize, Deserialize)]
pub struct TodoItemNew {
    pub title: String,
}

#[table_name = "tasks"]
#[derive(juniper::GraphQLInputObject, Deserialize, AsChangeset)]
pub struct TodoItemEdit {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[table_name = "tasks"]
#[derive(juniper::GraphQLInputObject, Deserialize, AsChangeset)]
pub struct TodoItemReplacement {
    pub title: String,
    pub completed: bool,
}
