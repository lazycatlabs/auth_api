use actix_web::{web, web::Json, HttpRequest, HttpResponse};

use crate::{
    core::{
        middlewares::{auth::AuthMiddleware, state::AppState},
        response::ResponseBody,
        types::AppResult,
    },
    features::auth::domain::usecases::{general_token::*, login::*, login_session::*, logout::*},
};

use super::domain::usecases::{dto::UpdatePasswordParams, interface::IAuthService};

pub async fn general_token_controller(
    state: web::Data<AppState>,
    params: Json<GeneralTokenParams>,
) -> AppResult<HttpResponse> {
    general_token(&state.di_container.auth_repository, params.0)
        .map(|data| ResponseBody::success(Some(data)).into())
}

pub async fn login_contoller(
    state: web::Data<AppState>,
    params: Json<LoginParams>,
    req: HttpRequest,
    // _: GeneralMiddleware,
) -> AppResult<HttpResponse> {
    let ip_addr = req.peer_addr().unwrap().ip().to_string();
    let new_params = LoginParams {
        ip_addr: Some(ip_addr),
        ..params.into_inner()
    };
    login(&state.di_container.auth_repository, new_params)
        .map(|data| ResponseBody::success(Some(data)).into())
}

pub async fn logout_controller(
    state: web::Data<AppState>,
    auth: AuthMiddleware,
) -> AppResult<HttpResponse> {
    logout(
        &state.di_container.auth_repository,
        auth.user.id,
        auth.login_session,
    )
    .map(|_| ResponseBody::<()>::success(None).into())
}

pub async fn login_session_controller(
    state: web::Data<AppState>,
    auth: AuthMiddleware,
) -> AppResult<HttpResponse> {
    login_session(&state.di_container.auth_repository, auth.user.id)
        .map(|data| ResponseBody::success(Some(data)).into())
}

pub async fn update_password(
    state: web::Data<AppState>,
    params: Json<UpdatePasswordParams>,
    auth: AuthMiddleware,
) -> AppResult<HttpResponse> {
    state
        .di_container
        .auth_service
        .update_password(auth.user.id, params.0)
        .map(|_| ResponseBody::<()>::success(None).into())
}
