use crate::app_state::AppState;
use crate::models::task::{TodoItem, TodoItemEdit, TodoItemNew};
use crate::services::TaskRepository;
use diesel::result::{Error, QueryResult};
use juniper::{FieldResult, RootNode};

/// Context
impl juniper::Context for AppState {}

pub struct Query {}

/// Query
#[juniper::object(Context = AppState)]
impl Query {
    pub async fn todos(context: &AppState) -> FieldResult<Vec<TodoItem>> {
        let conn = context.pool.get().unwrap();
        let taskService = TaskRepository::new(&conn);
        let result = taskService.list()?;
        Result::Ok(result)
    }

    pub async fn todo(context: &AppState, id: String) -> FieldResult<TodoItem> {
        let conn = context.pool.get().unwrap();
        let task_service = TaskRepository::new(&conn);
        let result = task_service.findById(id)?;
        Result::Ok(result)
    }
}

/// Mutation
pub struct Mutation {}

#[juniper::object(Context = AppState)]
impl Mutation {
    pub async fn create_todo(input: TodoItemNew, context: &AppState) -> FieldResult<TodoItem> {
        let conn = context.pool.get().unwrap();
        let task_service = TaskRepository::new(&conn);
        let result = task_service.create(input)?;
        Result::Ok(result)
    }

    pub async fn update_todo(
        id: String,
        input: TodoItemEdit,
        context: &AppState,
    ) -> FieldResult<TodoItem> {
        let conn = context.pool.get().unwrap();
        let task_service = TaskRepository::new(&conn);
        let result = task_service.edit(id, input)?;
        Result::Ok(result)
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
