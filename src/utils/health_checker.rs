use actix_web::HttpResponse;

use crate::core::{
    response::ResponseBody,
    types::AppResult,
};
use crate::core::middlewares::general::GeneralMiddleware;

pub async fn health_checker(_: GeneralMiddleware) -> AppResult<HttpResponse> {
    Ok(ResponseBody::<()>::success(None).into())
}