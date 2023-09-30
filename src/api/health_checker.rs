use actix_web::HttpResponse;

use crate::{
    error::ServiceError,
    models::response::{Diagnostic, ResponseBodyNoData},
    models::user::User,
};

pub async fn health_checker(user: User) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json(
        ResponseBodyNoData::new(
            Diagnostic::new("200", &user.id.to_string()))))
}