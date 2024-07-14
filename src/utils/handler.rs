use actix_web::HttpResponse;

use crate::core::response::{Diagnostic, ResponseBody};

pub async fn route_not_found() -> HttpResponse {
    ResponseBody::<()>::new(Diagnostic::new("404", "Route not found"), None).into()
}
