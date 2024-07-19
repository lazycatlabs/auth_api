use actix_web::{
    web::{self, Json},
    HttpResponse,
};

use crate::utils::mail_sender::send_email;

use crate::core::{middlewares::state::AppState, response::ResponseBody, types::AppResult};
use super::domain::usecase::dto::SendEmailParams;

pub async fn test_email(
    _: web::Data<AppState>,
    params: Json<SendEmailParams>,
) -> AppResult<HttpResponse> {
    send_email(params.0).await.map(|_| ResponseBody::<()>::success(None).into())
}
