pub mod task_handlers;
pub mod user_handlers;

pub use task_handlers::todo_service;
pub use user_handlers::user_service;

use crate::models::user::UserClaims;
use crate::services::user_service::UserService;
use actix_web::error;
use actix_web::error::{ErrorBadRequest, ErrorUnauthorized};
use actix_web::{dev, http, web, FromRequest, HttpRequest, HttpResponse, Result};
use futures::future;
use futures::future::Ready;

impl FromRequest for UserClaims {
    type Error = error::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        let result = get_user_claims(req);
        match result {
            Ok(claims) => future::ok(claims),
            Err(err) => future::err(err),
        }
    }
}

fn get_user_claims(req: &HttpRequest) -> Result<UserClaims, error::Error> {
    let authHeader = req
        .headers()
        .get("Authorization")
        .ok_or(ErrorUnauthorized("Required auth token"))?
        .to_str()
        .unwrap()
        .to_string();

    let words: Vec<&str> = authHeader.split(' ').collect();
    let token = words[1];
    UserService::decode_token(token).map_err(|x| ErrorBadRequest(format!("error code: {}", x)))
}
