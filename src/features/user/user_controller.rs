use actix_web::web::Json;
use actix_web::{web, HttpResponse};

use crate::{
    core::{
        constants::STATUS_SUCCESS,
        middlewares::{auth::AuthMiddleware, general::GeneralMiddleware, state::AppState},
        response::{Diagnostic, PageInfo, ResponseBody},
        types::AppResult,
    },
    features::user::domain::usecase::{
        delete_user::*, dto::PaginationParams, interface::IUserService, register::*, update_user::*,
    },
};

pub async fn register_controller(
    _: GeneralMiddleware,
    state: web::Data<AppState>,
    params: Json<RegisterParams>,
) -> AppResult<HttpResponse> {
    register(&state.di_container.user_repository, params.0)
        .map(|data| ResponseBody::success(Some(data)).into())
}

pub async fn get_user(auth: AuthMiddleware) -> AppResult<HttpResponse> {
    Ok(ResponseBody::success(Some(auth.user)).into())
}

pub async fn update_user_controller(
    auth: AuthMiddleware,
    state: web::Data<AppState>,
    params: Json<UpdateUserParams>,
) -> AppResult<HttpResponse> {
    update_user(&state.di_container.user_repository, auth.user.id, params.0)
        .map(|data| ResponseBody::success(Some(data)).into())
}

pub async fn delete_user_controller(
    auth: AuthMiddleware,
    state: web::Data<AppState>,
) -> AppResult<HttpResponse> {
    delete_user(&state.di_container.user_repository, auth.user.id).map(|data| {
        ResponseBody::<()>::new(Diagnostic::new(STATUS_SUCCESS, data.as_str()), None).into()
    })
}

pub async fn users(
    _: GeneralMiddleware,
    state: web::Data<AppState>,
    params: web::Query<PaginationParams>,
) -> AppResult<HttpResponse> {
    state.di_container.user_service.users(params.0).map(|data| {
        ResponseBody::success_pagination(
            Some(data.users),
            PageInfo::new(data.page, data.per_page, data.total),
        )
        .into()
    })
}
