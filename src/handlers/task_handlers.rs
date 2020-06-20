use crate::app_state::AppState;
use crate::models::task::{TodoItem, TodoItemEdit, TodoItemNew, TodoItemReplacement};
use crate::services::TaskRepository;
use actix_web::http::StatusCode;
use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let conn = data.pool.get().unwrap();
    let taskService = TaskRepository::new(&conn);
    let result = taskService.list();

    HttpResponse::Ok().json(result.unwrap())
}

pub async fn create_todo(
    info: web::Json<TodoItemNew>,
    data: web::Data<AppState>,
) -> impl Responder {
    let conn = data.pool.get().unwrap();
    let taskService = TaskRepository::new(&conn);
    let payload: TodoItemNew = info.into_inner();
    let result = taskService.create(payload);

    HttpResponse::build(StatusCode::from_u16(201).unwrap()).json(result.unwrap())
}

async fn update_todo(
    data: web::Data<AppState>,
    info: web::Json<TodoItemReplacement>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let conn = data.pool.get().unwrap();
    let taskService = TaskRepository::new(&conn);
    let result = taskService.update(id.to_string(), info.into_inner());

    HttpResponse::Ok().json(result.unwrap())
}

async fn edit_todo(
    data: web::Data<AppState>,
    info: web::Json<TodoItemEdit>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let conn = data.pool.get().unwrap();
    let taskService = TaskRepository::new(&conn);
    let result = taskService.edit(id.to_string(), info.into_inner());

    HttpResponse::Ok().json(result.unwrap())
}

async fn delete_todo(data: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    let conn = data.pool.get().unwrap();
    let taskService = TaskRepository::new(&conn);
    let result = taskService.delete(id.to_string());

    HttpResponse::Ok().json(result.unwrap())
}

pub fn todo_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(index))
            .route(web::post().to(create_todo))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );

    cfg.service(
        web::resource("/{item_id}")
            .route(web::put().to(update_todo))
            .route(web::patch().to(edit_todo))
            .route(web::delete().to(delete_todo)),
    );
}
