use actix_web::{HttpResponse, web};
use actix_web::web::Json;

use crate::{
    core::{
        middlewares::{
            auth::AuthMiddleware,
            state::AppState,
        },
        response::ResponseBody,
        types::AppResult,
    },
    features::user::domain::usecase::{
        dto::{
            RegisterParams,
            UpdateUserParams,
        },
        interface::IUserService,
    },
};

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

pub async fn update_user(
    auth: AuthMiddleware,
    state: web::Data<AppState>,
    params: Json<UpdateUserParams>,
) -> AppResult<HttpResponse> {
    match state.di_container.user_service
        .update_user(auth.user.id, params.0) {
        Ok(data) => Ok(ResponseBody::success(Some(data)).into()),
        Err(e) => Err(e),
    }
}