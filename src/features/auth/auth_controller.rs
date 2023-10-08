use actix_web::{HttpResponse, web};
use actix_web::web::Json;

use crate::{
    core::{
        config::state::AppState,
        constants::{MESSAGE_SUCCESS, STATUS_SUCCESS},
        response::{
            Diagnostic,
            ResponseBody,
        },
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
        Ok(data) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(
                Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
                Some(data),
            )
        )),
        Err(e) => Err(e),
    }
}