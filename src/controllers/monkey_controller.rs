use actix_web::{web, Responder, HttpResponse as response};
use crate::entities::request;
use crate::services::monkey_service;

pub async fn register(r: web::Json<request::MonkeyRegisterEntity>) -> impl Responder {
    match monkey_service::create(&r) {
        Ok(m) => return response::Ok().json(m),
        Err(e) => return response::InternalServerError().json(e.to_string()),
    };
}
