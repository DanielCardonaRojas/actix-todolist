use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod handlers;
mod middlewares;
mod models;

use handlers::*;
use middlewares::Logging;
use models::AppState;
use slog::{info, o, Drain};
use slog_term;

fn configure_log() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let log = configure_log();
    info!(log, "Starting server at: http://localhost:8088");
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                logger: log.clone(),
            })
            .wrap(Logging::new(log.clone()))
            .service(web::scope("/todos").configure(todo_service))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
