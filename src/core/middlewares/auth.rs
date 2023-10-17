use std::future::Future;
use std::pin::Pin;

use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::http::header::HeaderValue;
use base64::Engine;
use base64::engine::general_purpose;
use dotenv_codegen::dotenv;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use uuid::Uuid;

use crate::{
    core::{
        config::{
            db::init_db,
            di::DiContainer,
        },
        constants::AUTHORIZATION,
        error::APIError,
        types::AppResult,
    },
    features::{
        auth::{
            data::models::auth_token::AuthToken,
            domain::usecase::interface::IAuthService,
        },
        user::{
            domain::entity::user::UserEntity,
            domain::usecase::interface::IUserService,
        },
    },
};

pub struct AuthMiddleware {
    pub user: UserEntity,
    pub login_session: Uuid,
}


impl FromRequest for AuthMiddleware {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output=Result<AuthMiddleware, Self::Error>>>>;


    // act as auth middleware
    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = init_db();
        let di = DiContainer::new(&pool);
        if let Some(header_auth_string) = request.headers().get(AUTHORIZATION) {
            if let Ok(auth_str) = header_auth_string.to_str() {
                if is_auth_header_valid(header_auth_string) {
                    let token = token_extractor(&auth_str);
                    if let Ok(token_data) = decode_token(&token.to_string()) {
                        if let Ok(user_id) = di.auth_service.verify_token(&token_data) {
                            if let Ok(user) = di.user_service.find_user_by_id(user_id) {
                                return Box::pin(async move {
                                    Ok(AuthMiddleware {
                                        user,
                                        login_session: token_data.claims.login_session,
                                    })
                                });
                            }
                        }
                    }
                }
            }
        }
        Box::pin(async move { Err(APIError::Unauthorized) })
    }
}


pub fn decode_token(jwt: &String) -> AppResult<TokenData<AuthToken>> {
    let bytes_public_key = general_purpose::STANDARD.decode(dotenv!("ACCESS_TOKEN_PUBLIC_KEY")).unwrap();
    let decoded_public_key = String::from_utf8(bytes_public_key).unwrap();
    jsonwebtoken::decode::<AuthToken>(
        jwt,
        &DecodingKey::from_rsa_pem(decoded_public_key.as_bytes()).unwrap(),
        &Validation::new(Algorithm::RS256),
    ).map_err(|_e| APIError::Unauthorized)
}

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
