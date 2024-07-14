use actix_web::web::Json;
use actix_web::{web, HttpResponse};

use crate::{
    core::{
        constants::STATUS_SUCCESS,
        middlewares::{auth::AuthMiddleware, state::AppState},
        response::{Diagnostic, ResponseBody},
        types::AppResult,
    },
    features::user::domain::usecase::{
        dto::{RegisterParams, UpdateUserParams},
        interface::IUserService,
    },
};

pub async fn register(
    state: web::Data<AppState>,
    params: Json<RegisterParams>,
) -> AppResult<HttpResponse> {
    match state.di_container.user_service.register(params.0) {
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
    match state
        .di_container
        .user_service
        .update_user(auth.user.id, params.0)
    {
        Ok(data) => Ok(ResponseBody::success(Some(data)).into()),
        Err(e) => Err(e),
    }
}

pub async fn delete_user(
    auth: AuthMiddleware,
    state: web::Data<AppState>,
) -> AppResult<HttpResponse> {
    match state.di_container.user_service.delete_user(auth.user.id) {
        Ok(data) => Ok(ResponseBody::<()>::new(
            Diagnostic::new(STATUS_SUCCESS, data.as_str()),
            None,
        )
        .into()),
        Err(e) => Err(e),
    }
}
