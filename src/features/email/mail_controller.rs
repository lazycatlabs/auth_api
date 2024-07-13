use actix_web::{web, web::Json, HttpRequest, HttpResponse};

use crate::features::email::domain::usecase::dto::EmailParams;
use crate::{
    core::{
        middlewares::state::AppState,
        response::ResponseBody,
        types::AppResult,
    },
    features::auth::domain::usecase::interface::IAuthService,
};

pub async fn send_mail(
    state: web::Data<AppState>,
    params: Json<EmailParams>,
) -> AppResult<HttpResponse> {
    match state.di_container.auth_service.general_token(params.0) {
        Ok(data) => Ok(ResponseBody::success(Some(data)).into()),
        Err(e) => Err(e),
    }
}
