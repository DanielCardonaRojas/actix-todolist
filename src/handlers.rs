use crate::models::{TodoItem, TodoItemEdit, TodoItemNew};
use actix_web::http::StatusCode;
use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

pub async fn index() -> impl Responder {
    let items = [
        TodoItem {
            uuid: "123".to_string(),
            title: "task1".to_string(),
            completed: false,
        },
        TodoItem {
            uuid: "919".to_string(),
            title: "task2".to_string(),
            completed: true,
        },
    ];

    HttpResponse::Ok().json(items)
}

pub async fn create_todo(info: web::Json<TodoItemNew>) -> impl Responder {
    HttpResponse::build(StatusCode::from_u16(201).unwrap()).json(TodoItem {
        uuid: "123".to_string(),
        title: info.title.to_string(),
        completed: false,
    })
}

async fn update_todo(info: web::Json<TodoItemEdit>, id: web::Path<String>) -> impl Responder {
    let mut item = TodoItem {
        uuid: id.to_string(),
        title: "some task".to_string(),
        completed: false,
    };

    item.completed = info.completed.unwrap_or(item.completed);
    item.title = info
        .title
        .as_ref()
        .map(move |t| t.to_string())
        .unwrap_or(item.title);

    HttpResponse::Ok().json(item)
}

pub fn todo_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(index))
            .route(web::post().to(create_todo))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );

    cfg.service(web::resource("/{item_id}").route(web::put().to(update_todo)));
}
