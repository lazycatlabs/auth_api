use actix_web::HttpResponse;
use actix_web::web::{Data, Json};

use crate::core::{
    constants::{MESSAGE_SUCCESS, STATUS_SUCCESS},
    response::{Diagnostic, ResponseBodyNoData},
    types::AppResult,
};
use crate::features::user::domain::usecase::{
    dto::RegisterParams, interface::IUserService,
};

pub async fn register(
    user_service: Data<dyn IUserService>,
    params: Json<RegisterParams>,
) -> AppResult<HttpResponse> {
    let result = user_service.register(params.0).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json(
            ResponseBodyNoData::new(
                Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
            )
        )),
        Err(e) => Err(e),
    }
}