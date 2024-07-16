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
    state
        .di_container
        .user_service
        .register(params.0)
        .map(|_| ResponseBody::<()>::success(None).into())
}

pub async fn get_user(auth: AuthMiddleware) -> AppResult<HttpResponse> {
    Ok(ResponseBody::success(Some(auth.user)).into())
}

pub async fn update_user(
    auth: AuthMiddleware,
    state: web::Data<AppState>,
    params: Json<UpdateUserParams>,
) -> AppResult<HttpResponse> {
    state
        .di_container
        .user_service
        .update_user(auth.user.id, params.0)
        .map(|data| ResponseBody::success(Some(data)).into())
}

pub async fn delete_user(
    auth: AuthMiddleware,
    state: web::Data<AppState>,
) -> AppResult<HttpResponse> {
    state
        .di_container
        .user_service
        .delete_user(auth.user.id)
        .map(|data| {
            ResponseBody::<()>::new(Diagnostic::new(STATUS_SUCCESS, data.as_str()), None).into()
        })
}
