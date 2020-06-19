use super::schema::{tasks, users};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    #[serde(skip)] // we're removing id from being show in the response
    pub id: String,
    pub email: String,
    #[serde(skip)] // we're removing password from being show in the response
    pub password: String,
    pub created_at: NaiveDateTime,
}

impl User {
    fn new(password: String, email: String) -> Self {
        User {
            id: Uuid::new_v4().to_string(),
            email: email,
            password: bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap(),
            created_at: Local::now().naive_utc(),
        }
    }
}

impl From<RegisterUser> for User {
    fn from(registration: RegisterUser) -> Self {
        User::new(registration.password, registration.email)
    }
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserClaims {
    pub email: String,
    pub id: String,
}

impl From<User> for UserClaims {
    fn from(user: User) -> Self {
        UserClaims {
            email: user.email,
            id: user.id,
        }
    }
}

#[table_name = "tasks"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
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

#[derive(Serialize, Deserialize)]
pub struct TodoItemNew {
    pub title: String,
}

#[table_name = "tasks"]
#[derive(Deserialize, AsChangeset)]
pub struct TodoItemEdit {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[table_name = "tasks"]
#[derive(Deserialize, AsChangeset)]
pub struct TodoItemReplacement {
    pub title: String,
    pub completed: bool,
}
