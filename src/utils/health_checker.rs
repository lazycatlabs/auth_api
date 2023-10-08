use actix_web::HttpResponse;

use crate::core::{
    response::ResponseBody,
    types::AppResult,
};

pub async fn health_checker() -> AppResult<HttpResponse> {
    Ok(ResponseBody::<()>::success(None).into())
}