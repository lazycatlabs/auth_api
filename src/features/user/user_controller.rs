use actix_web::{HttpResponse, web};
use actix_web::web::Json;

use crate::{
    core::{
        middlewares::state::AppState,
        response::ResponseBody,
        types::AppResult,
    },
    features::user::domain::usecase::{
        dto::RegisterParams,
        interface::IUserService,
    },
};
use crate::core::middlewares::auth::AuthMiddleware;

pub async fn register(
    state: web::Data<AppState>,
    params: Json<RegisterParams>,
) -> AppResult<HttpResponse> {
    let result = state.di_container.user_service.register(params.0).await;

    match result {
        Ok(_) => Ok(ResponseBody::<()>::success(None).into()),
        Err(e) => Err(e),
    }
}

pub async fn get_user(auth: AuthMiddleware) -> AppResult<HttpResponse> {
    Ok(ResponseBody::success(Some(auth.user)).into())
}