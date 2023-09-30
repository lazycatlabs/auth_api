use core::future::Future;
use std::pin::Pin;

use actix_web::{FromRequest, get, HttpRequest, HttpResponse};
use actix_web::dev::Payload;
use actix_web::web::Data;

use crate::config::db::Pool;
use crate::error::ServiceError;
use crate::models::jwt::UserToken;
use crate::models::response::{Diagnostic, ResponseBodyNoData};
use crate::models::user::User;
use crate::utils::token_utils;
use crate::utils::token_utils::token_extractor;

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

#[get("/health_checker")]
async fn health_checker(test: HealthChecker) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json(
        ResponseBodyNoData::new(Diagnostic::new("200", &test.status))))
}

impl FromRequest for HealthChecker {
    type Error = ServiceError;
    type Future = Pin<Box<dyn Future<Output=Result<HealthChecker, Self::Error>>>>;


    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(header_auth_string) = request.headers().get("Authorization") {
            if let Some(pool) = request.app_data::<Data<Pool>>() {
                if let Ok(auth_str) = header_auth_string.to_str() {
                    if token_utils::is_auth_header_valid(header_auth_string) {
                        let token = token_extractor(&auth_str);
                        if let Ok(token_data) = UserToken::decode_token(&token.to_string()) {
                            if let Ok(user_id) = token_utils::verify_token(&token_data, pool) {
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
