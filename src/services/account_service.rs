use actix_web::web;
use uuid::Uuid;

use crate::
{config::db::Pool,
 error::ServiceError, models::{
    jwt::UserToken,
    user::{LoginDTO, User, UserDTO},
}};
use crate::models::jwt::JWTResponse;

pub fn signup(user_new: UserDTO, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    match User::signup(user_new, &mut pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::BadRequest { message })
    }
}

pub fn login(user: LoginDTO, pool: &web::Data<Pool>) -> Result<JWTResponse, ServiceError> {
    match User::login(user, &mut pool.get().unwrap()) {
        Ok(logged_user) => {
            let generate_token_str = UserToken::generate_token(&logged_user);
            match generate_token_str {
                Ok(token_res) => {
                    if logged_user.login_session.is_empty() {
                        Err(ServiceError::Unauthorized)
                    } else {
                        Ok(JWTResponse::new(token_res))
                    }
                }
                Err(_) => Err(ServiceError::InternalError),
            }
        }
        Err(_) => Err(ServiceError::Unauthorized),
    }
}

pub fn logout(user_id: Uuid, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    User::logout(user_id, &mut pool.get().unwrap());
    return Ok(());
}