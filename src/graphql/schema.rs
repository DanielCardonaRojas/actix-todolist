use crate::app_state::AppState;
use crate::models::task::TodoItem;
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
}

/// Mutation
pub struct Mutation {}
#[juniper::object(Context = AppState)]
impl Mutation {}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
