use actix_web::{get, HttpResponse, Responder, web::ServiceConfig};
use serde::Serialize;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(ping);
}

#[derive(Serialize, Debug)]
struct Pong {
    status: String,
    code: i16,
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(Pong {
        status: "ok".to_string(),
        code: 200,
    })
}