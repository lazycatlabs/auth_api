use actix_web::{HttpRequest, HttpResponse, web};

use crate::{
    config::db::Pool,
    constants::{*},
    error::ServiceError,
    models::{
        response::{Diagnostic, ResponseBody, ResponseBodyNoData},
        user::{LoginDTO, UserDTO},
    },
    services::account_service,
};

pub async fn signup(
    user: web::Json<UserDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match account_service::signup(user.0, &pool) {
        Ok(_) => Ok(HttpResponse::Ok()
            .json(
                ResponseBodyNoData::new(
                    Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
                )
            )),
        Err(err) => Err(err)
    }
}

pub async fn login(
    login: web::Json<LoginDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match account_service::login(login.0, &pool) {
        Ok(token_response) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
            token_response,
        ))),
        Err(err) => Err(err)
    }
}

pub async fn logout(
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        match account_service::logout(auth_header, &pool) {
            Ok(_) => Ok(HttpResponse::Ok().json(ResponseBodyNoData::new(
                Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
            ))),
            Err(err) => Err(err)
        }
    } else {
        Err(ServiceError::BadRequest {
            message: MESSAGE_TOKEN_MISSING.to_string()
        })
    }
}

pub async fn profile(
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        match account_service::profile(auth_header, &pool) {
            Ok(user) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
                user,
            ))),
            Err(err) => Err(err)
        }
    } else {
        Err(ServiceError::BadRequest {
            message: MESSAGE_TOKEN_MISSING.to_string()
        })
    }
}
