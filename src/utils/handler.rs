use actix_web::{http::StatusCode, HttpResponse};

use crate::core::response::{Diagnostic, ResponseBody};

pub async fn route_not_found() -> HttpResponse {
    HttpResponse::build(StatusCode::NOT_FOUND).json(ResponseBody::<()>::new(Diagnostic::new("404", "Route not found"), None))
}
