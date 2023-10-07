use actix_web::HttpResponse;
use actix_web::web::{Data, Json};

use crate::core::{
    constants::{MESSAGE_SUCCESS, STATUS_SUCCESS},
    response::Diagnostic,
    types::AppResult,
};
use crate::core::response::ResponseBody;
use crate::features::auth::domain::usecase::{
    dto::LoginParams, interface::IAuthService,
};

pub async fn login(
    user_service: Data<dyn IAuthService>,
    params: Json<LoginParams>,
) -> AppResult<HttpResponse> {
    let result = user_service.login(params.0).await;

    match result {
        Ok(data) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(
                Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
                data,
            )
        )),
        Err(e) => Err(e),
    }
}