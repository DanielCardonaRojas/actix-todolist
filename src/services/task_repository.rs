use crate::db_connection::{PgPool, PgPooledConnection};
use crate::diesel::RunQueryDsl;
use crate::models::task::{TodoItem, TodoItemEdit, TodoItemNew, TodoItemReplacement};
use crate::schema::tasks;
use crate::schema::tasks::dsl::*;
use diesel::prelude::*;
use diesel::result::{Error, QueryResult};

pub struct TaskRepository<'a> {
    connection: &'a PgPooledConnection,
}

impl<'a> TaskRepository<'a> {
    pub fn new(connection: &'a PgPooledConnection) -> TaskRepository {
        Self {
            connection: connection,
        }
    }

    pub fn create(&self, input: TodoItemNew) -> QueryResult<TodoItem> {
        return diesel::insert_into(tasks::table)
            .values(TodoItem::from(input))
            .get_result(self.connection);
    }

    pub fn delete(&self, key: String) -> QueryResult<usize> {
        diesel::delete(tasks.filter(id.eq(key))).execute(self.connection)
    }

    pub fn update(&self, key: String, input: TodoItemReplacement) -> QueryResult<TodoItem> {
        diesel::update(tasks)
            .filter(id.eq(key))
            .set(input)
            .get_result(self.connection)
    }

    pub fn findById(&self, key: String) -> QueryResult<TodoItem> {
        tasks::table
            .find(key)
            .get_result::<TodoItem>(self.connection)
    }

    pub fn list(&self) -> QueryResult<Vec<TodoItem>> {
        let all = tasks::table.load::<TodoItem>(self.connection)?;
        Result::Ok(all)
    }

    pub fn edit(&self, key: String, input: TodoItemEdit) -> QueryResult<TodoItem> {
        diesel::update(tasks)
            .filter(id.eq(key))
            .set(input)
            .get_result(self.connection)
    }
}
