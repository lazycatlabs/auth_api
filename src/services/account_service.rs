use actix_web::http::header::HeaderValue;
use actix_web::web;

use crate::
{config::db::Pool,
 constants::*,
 error::ServiceError, models::{
    jwt::UserToken,
    user::{LoginDTO, User, UserDTO},
}};
use crate::models::jwt::JWTResponse;
use crate::utils::token_utils;
use crate::utils::token_utils::token_extractor;

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

pub fn logout(auth_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    if let Ok(auth_str) = auth_header.to_str() {
        if token_utils::is_auth_header_valid(auth_header) {
            let token = token_extractor(&auth_str);
            if let Ok(token_data) = UserToken::decode_token(&token.to_string()) {
                if let Ok(id) = token_utils::verify_token(&token_data, pool) {
                    if let Ok(user) = User::find_user_by_id(&id, &mut pool.get().unwrap()) {
                        User::logout(user.id, &mut pool.get().unwrap());
                        return Ok(());
                    }
                }
            } else {
                return Err(ServiceError::Unauthorized);
            }
        } else {
            return Err(ServiceError::BadRequest {
                message: MESSAGE_BAD_REQUEST.to_string(),
            });
        }
    }
    return Err(ServiceError::Unauthorized);
}

pub fn profile(auth_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<User, ServiceError> {
    if let Ok(auth_str) = auth_header.to_str() {
        if token_utils::is_auth_header_valid(auth_header) {
            let token = token_extractor(&auth_str);
            if let Ok(token_data) = UserToken::decode_token(&token.to_string()) {
                if let Ok(user_id) = token_utils::verify_token(&token_data, pool) {
                    if let Ok(user) = User::find_user_by_id(&user_id, &mut pool.get().unwrap()) {
                        return Ok(user);
                    }
                }
            } else {
                return Err(ServiceError::Unauthorized);
            }
        } else {
            return Err(ServiceError::BadRequest {
                message: MESSAGE_BAD_REQUEST.to_string(),
            });
        }
    }
    return Err(ServiceError::Unauthorized);
}