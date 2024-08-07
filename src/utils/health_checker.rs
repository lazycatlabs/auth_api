use actix_web::HttpResponse;

use crate::core::{
    middlewares::general::GeneralMiddleware, response::ResponseBody, types::AppResult,
};

pub async fn health_checker(_: GeneralMiddleware) -> AppResult<HttpResponse> {
    Ok(ResponseBody::<()>::success(None).into())
}
