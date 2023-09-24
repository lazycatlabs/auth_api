use actix_web::{get, HttpResponse, Responder};

use crate::models::response::{Diagnostic, ResponseBodyNoData};

#[get("/health_checker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "Hello, Lzyct";

    HttpResponse::Ok().json(
        ResponseBodyNoData::new(Diagnostic::new("200", MESSAGE)))
}

