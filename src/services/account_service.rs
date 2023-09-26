use actix_web::web;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::{
        jwt::UserToken,
        user::{LoginDTO, User, UserDTO},
    },
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub fn signup(user_new: UserDTO, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    match User::signup(user_new, &mut pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::BadRequest { message })
    }
}

pub fn login(user: LoginDTO, pool: &web::Data<Pool>) -> Result<TokenBodyResponse, ServiceError> {
    match User::login(user, &mut pool.get().unwrap()) {
        Ok(logged_user) => {
            let generate_token_str = UserToken::generate_token(&logged_user).unwrap();
            match serde_json::from_value(
                json!({ "token": generate_token_str, "tokenType": "Bearer" }),
            ) {
                Ok(token_res) => {
                    if logged_user.login_session.is_empty() {
                        Err(ServiceError::Unauthorized)
                    } else {
                        Ok(token_res)
                    }
                }
                Err(_) => Err(ServiceError::InternalError),
            }
        }
        Err(_) => Err(ServiceError::Unauthorized),
    }
}