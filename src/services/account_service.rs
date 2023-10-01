use actix_web::web;
use uuid::Uuid;

use crate::
{config::db::Pool,
 error::ServiceError, models::{
    jwt::UserToken,
    user::{LoginDTO, User, UserDTO},
}};
use crate::models::jwt::JWTResponse;
use crate::models::user::UpdateUserDTO;

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
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

pub fn logout(user_id: Uuid, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    User::logout(user_id, &mut pool.get().unwrap());
    return Ok(());
}

pub fn update_user(
    user_id: Uuid,
    user_update: UpdateUserDTO,
    pool: &web::Data<Pool>,
) -> Result<User, ServiceError> {
    match User::update_user(user_id, user_update, &mut pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(err) => Err(err)
    }
}

pub fn delete_user(
    user_id: Uuid,
    pool: &web::Data<Pool>,
) -> Result<String, ServiceError> {
    match User::delete_user(user_id, &mut pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(err) => Err(err)
    }
}