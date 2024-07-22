use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use validator::Validate;

use crate::{
    core::{error::APIError, middlewares::general::GeneralMiddleware},
    features::general::domain::usecase::dto::SendEmailParams,
};
use crate::{
    core::{middlewares::state::AppState, response::ResponseBody, types::AppResult},
    utils::mail_sender::send_email,
};

pub async fn test_email(
    _: GeneralMiddleware,
    __: web::Data<AppState>,
    params: Json<SendEmailParams>,
) -> AppResult<HttpResponse> {
    params.validate().map_err(|e| APIError::BadRequest {
        message: e.to_string(),
    })?;
    send_email(params.0)
        .await
        .map(|_| ResponseBody::<()>::success(None).into())
}
