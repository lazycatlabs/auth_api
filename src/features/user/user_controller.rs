use actix_web::{HttpResponse, web};
use actix_web::web::Json;

use crate::core::{
    config::state::AppState,
    constants::{MESSAGE_SUCCESS, STATUS_SUCCESS},
    response::Diagnostic,
    types::AppResult,
};
use crate::core::response::ResponseBody;
use crate::features::user::domain::usecase::{
    dto::RegisterParams,
    interface::IUserService,
};

pub async fn register(
    state: web::Data<AppState>,
    params: Json<RegisterParams>,
) -> AppResult<HttpResponse> {
    let result = state.di_container.user_service.register(params.0).await;

    match result {
        Ok(_) => Ok(ResponseBody::<()>::new(
            Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
            None,
        ).into()),
        Err(e) => Err(e),
    }
}