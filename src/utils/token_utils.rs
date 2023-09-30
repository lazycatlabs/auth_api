use actix_web::http::header::HeaderValue;
use actix_web::web;
use jsonwebtoken::TokenData;
use uuid::Uuid;

use crate::{
    config::db::Pool,
    models::{
        jwt::UserToken,
        user::User,
    },
};

pub fn token_extractor(auth: &str) -> String {
    let bearer_str = auth.split(" ").collect::<Vec<&str>>();
    let token_prefix = bearer_str[1].split(".").collect::<Vec<&str>>();
    let token = token_prefix[1..].join(".");
    token
}


pub fn is_auth_header_valid(auth_header: &HeaderValue) -> bool {
    if let Ok(auth_str) = auth_header.to_str() {
        return auth_str.starts_with("bearer") || auth_str.starts_with("Bearer");
    }
    false
}

pub fn verify_token(jwt: &TokenData<UserToken>, pool: &web::Data<Pool>) -> Result<Uuid, String> {
    if User::is_valid_login_session(&jwt.claims, &mut pool.get().unwrap()) {
        Ok(jwt.claims.jti)
    } else {
        Err("Invalid token".to_string())
    }
}