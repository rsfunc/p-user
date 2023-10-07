use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    first_name: String,
    last_name: String,
}

#[actix_web::put("")]
async fn create(payload: web::Json<Payload>) -> impl Responder {
    HttpResponse::Ok().body(format!("Got: {payload:?}"))
}

#[actix_web::get("")]
async fn get() -> impl Responder {
    let p = Payload {
        first_name: String::from("Rex"),
        last_name: String::from("Weerd"),
    };
    actix_web::web::Json(p)
}
