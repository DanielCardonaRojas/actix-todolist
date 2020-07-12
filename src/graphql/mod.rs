use actix_web::{web, HttpResponse};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
mod schema;
use crate::app_state::AppState;
use schema::{create_schema, Schema};

async fn graphiql() -> HttpResponse {
    print!("Getting graphql docs");
    let html = graphiql_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    data: web::Json<GraphQLRequest>,
    schema: web::Data<Schema>,
    context: web::Data<AppState>,
) -> HttpResponse {
    let res = data.execute(&schema, &context);
    HttpResponse::Ok().json(res)
}

pub fn graphql_service(config: &mut web::ServiceConfig) {
    let schema = create_schema();

    config.data(schema).service(
        web::resource("")
            .route(web::get().to(graphiql))
            .route(web::post().to(graphql)),
    );
}
