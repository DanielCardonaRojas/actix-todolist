use crate::app_state::AppState;
use crate::models::user::{AuthUser, RegisterUser, UserClaims};
use crate::services::UserService;
use actix_web::http::StatusCode;
use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

pub async fn login(
    authentication: web::Json<AuthUser>,
    data: web::Data<AppState>,
) -> impl Responder {
    let conn = data.pool.get().unwrap();
    let user_service = UserService::new(&conn);
    let user = user_service.login(authentication.into_inner()).unwrap();
    let claims = UserClaims::from(user);
    let token = UserService::create_token(claims).unwrap();
    let json_token = json!({ "token": token });
    HttpResponse::Ok().json(json_token)
}

pub async fn register(
    registration: web::Json<RegisterUser>,
    data: web::Data<AppState>,
) -> impl Responder {
    let conn = data.pool.get().unwrap();
    let user_service = UserService::new(&conn);
    let result = user_service.register(registration.into_inner());
    HttpResponse::Ok().json(result.unwrap())
}

pub fn user_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/register")
            .route(web::post().to(register))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );

    cfg.service(web::resource("/auth").route(web::post().to(login)));
}
