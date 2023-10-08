use actix_web::{HttpResponse, web};
use actix_web::web::Json;

use crate::{
    core::{
        middlewares::state::AppState,
        response::ResponseBody,
        types::AppResult,
    },
    features::auth::{
        domain::{
            usecase::{
                dto::LoginParams,
                interface::IAuthService,
            },
        },
    },
};

pub async fn login(
    state: web::Data<AppState>,
    params: Json<LoginParams>,
) -> AppResult<HttpResponse> {
    let result = state.di_container.auth_service.login(params.0).await;

    match result {
        Ok(data) => Ok(ResponseBody::success(Some(data)).into()),
        Err(e) => Err(e),
    }
}

pub async fn logout(
    state: web::Data<AppState>,
    auth: crate::core::middlewares::auth::AuthMiddleware,
) -> AppResult<HttpResponse> {
    let result = state.di_container.auth_service.logout(auth.user.id);

    match result {
        Ok(_) => Ok(ResponseBody::<()>::success(None).into()),
        Err(e) => Err(e),
    }
}