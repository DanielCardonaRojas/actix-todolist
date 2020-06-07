use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod handlers;
mod middlewares;
mod models;

use dotenv::dotenv;
use handlers::*;
use listenfd::ListenFd;
use middlewares::Logging;
use models::AppState;
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

    info!(log, "Starting server at: http://{}", base_url);
    let mut server = HttpServer::new(move || {
        App::new()
            .data(AppState {
                logger: log.clone(),
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

    lazy_static! {
        static ref APP_STATE: AppState = {
            AppState {
                logger: configure_log(),
            }
        };
    }

    #[test]
    fn test_str_len() {
        assert_eq!(2 + 2, 4);
    }

    #[actix_rt::test]
    async fn test_create_todos() {
        let mut app = test::init_service(
            App::new()
                .data(AppState {
                    logger: configure_log(),
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
