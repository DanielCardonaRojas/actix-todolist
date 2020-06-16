use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod app_state;
mod db_connection;
mod handlers;
mod middlewares;
mod models;
mod task_repository;

pub mod schema;
use dotenv::dotenv;
#[macro_use]
extern crate diesel;

use app_state::AppState;
use db_connection::init_pool;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use handlers::*;
use listenfd::ListenFd;
use middlewares::Logging;
use slog::{info, o, Drain};
use slog_term;
use std::env;

fn configure_log() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    let log = configure_log();
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();
    let base_url = format!("{}:{}", host, port);
    let mut listenfd = ListenFd::from_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_pool(&database_url).expect("Failed to create pool");

    info!(log, "Starting server at: http://{}", base_url);
    let mut server = HttpServer::new(move || {
        App::new()
            .data(AppState {
                logger: log.clone(),
                pool: pool.clone(),
            })
            .wrap(Logging::new(log.clone()))
            .service(web::scope("/todos").configure(todo_service))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind(base_url)?
    };

    server.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use handlers;
    use lazy_static::lazy_static;
    use serde_json::json;

    #[test]
    fn test_str_len() {
        assert_eq!(2 + 2, 4);
    }

    #[actix_rt::test]
    async fn test_create_todos() {
        let pool = init_pool("").expect("Failed to create pool");
        let mut app = test::init_service(
            App::new()
                .data(AppState {
                    logger: configure_log(),
                    pool: pool.clone(),
                })
                .service(web::scope("/todos").configure(todo_service)),
        )
        .await;

        let todo_title = "Create todo List";

        let create_todo_list = json!({ "title": todo_title });

        let req = test::TestRequest::post()
            .uri("/todos")
            .header("Content-Type", "application/json")
            .set_payload(create_todo_list.to_string())
            .to_request();

        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 201, "Status should be 200.");
    }
}
