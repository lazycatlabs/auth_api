use std::future::Future;
use std::pin::Pin;

use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::http::header::HeaderValue;
use base64::Engine;
use base64::engine::general_purpose;
use dotenv_codegen::dotenv;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};

use crate::{
    core::{
        constants::AUTHORIZATION,
        error::APIError,
        types::AppResult,
    },
    features::auth::data::models::general_token::GeneralToken,
};

pub struct GeneralMiddleware {
    pub data: TokenData<GeneralToken>,
}


impl FromRequest for GeneralMiddleware {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output=Result<GeneralMiddleware, Self::Error>>>>;


    // act as auth middleware
    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(header_auth_string) = request.headers().get(AUTHORIZATION) {
            if let Ok(auth_str) = header_auth_string.to_str() {
                if is_auth_header_valid(header_auth_string) {
                    let token = token_extractor(&auth_str);
                    if let Ok(token_data) = decode_token(&token.to_string()) {
                        return Box::pin(async move {
                            Ok(GeneralMiddleware { data: token_data })
                        });
                    }
                }
            }
        }
        Box::pin(async move { Err(APIError::Unauthorized) })
    }
}


pub fn decode_token(jwt: &String) -> AppResult<TokenData<GeneralToken>> {
    let bytes_public_key = general_purpose::STANDARD.decode(dotenv!("GENERAL_TOKEN_PUBLIC_KEY")).unwrap();
    let decoded_public_key = String::from_utf8(bytes_public_key).unwrap();
    jsonwebtoken::decode::<GeneralToken>(
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
