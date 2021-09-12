use actix_web::{HttpResponse, Responder};

pub async fn root() -> impl Responder {
    HttpResponse::Ok().json("OK")
}