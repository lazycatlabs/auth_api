use core::future::Future;
use std::pin::Pin;

use actix_web::{
    dev::Payload,
    FromRequest,
    HttpRequest,
    HttpResponse,
    web::Data,
};

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::response::{Diagnostic, ResponseBodyNoData},
    models::user::User,
    utils::token_utils::*,
};

pub struct HealthChecker {
    pub status: String,
}

impl HealthChecker {
    pub fn new(status: String) -> Self {
        Self {
            status
        }
    }
}

pub async fn health_checker(health_checker: HealthChecker) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json(
        ResponseBodyNoData::new(
            Diagnostic::new("200", &health_checker.status))))
}

impl FromRequest for HealthChecker {
    type Error = ServiceError;
    type Future = Pin<Box<dyn Future<Output=Result<HealthChecker, Self::Error>>>>;


    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(header_auth_string) = request.headers().get("Authorization") {
            if let Some(pool) = request.app_data::<Data<Pool>>() {
                if let Ok(auth_str) = header_auth_string.to_str() {
                    if is_auth_header_valid(header_auth_string) {
                        let token = token_extractor(&auth_str);
                        if let Ok(token_data) = decode_token(&token.to_string()) {
                            if let Ok(user_id) = verify_token(&token_data, pool) {
                                if let Ok(user) = User::find_user_by_id(&user_id, &mut pool.get().unwrap()) {
                                    return Box::pin(async move {
                                        Ok(HealthChecker::new(format!("Hello, {}", user.id)))
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }


        Box::pin(async move { Err(ServiceError::Unauthorized) })
    }
}
