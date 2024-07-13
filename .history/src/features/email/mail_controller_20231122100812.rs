use actix_web::{HttpRequest, HttpResponse, web, web::Json};

use crate::{
    core::{
        middlewares::{
            auth::AuthMiddleware,
            state::AppState,
        },
        response::ResponseBody,
        types::AppResult,
    },
    features::auth::{
        domain::{
            usecase::{
                dto::*,
                interface::IAuthService,
            },
        },
    },
};
use crate::features::email::domain::usecase::dto::EmailParams;

pub async fn send_mail(
    state: web::Data<AppState>,
    params: Json<EmailParams>,
) -> AppResult<HttpResponse> {
    match state.di_container.auth_service.general_token(params.0) {
        Ok(data) => Ok(ResponseBody::success(Some(data)).into()),
        Err(e) => Err(e),
    }
}