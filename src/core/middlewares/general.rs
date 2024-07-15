use std::future::Future;
use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::http::header::HeaderValue;
use actix_web::{FromRequest, HttpRequest};
use base64::engine::general_purpose;
use base64::Engine;
use dotenv_codegen::dotenv;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};

use crate::{
    core::{constants::AUTHORIZATION, error::APIError, types::AppResult},
    features::auth::data::models::general_token::GeneralToken,
};

pub struct GeneralMiddleware {
    pub data: TokenData<GeneralToken>,
}

impl FromRequest for GeneralMiddleware {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output = Result<GeneralMiddleware, Self::Error>>>>;

    // act as auth middleware
    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        let header_auth = request
            .headers()
            .get(AUTHORIZATION)
            .ok_or_else(|| APIError::Unauthorized);
        // check if auth header is valid
        if !is_auth_header_valid(header_auth.as_ref().unwrap()) {
            return Box::pin(async move { Err(APIError::Unauthorized) });
        }
        let auth_str =
            header_auth.and_then(|auth| auth.to_str().map_err(|_| APIError::Unauthorized));

        let token = token_extractor(&auth_str.unwrap());
        let token_data = decode_token(&token.to_string()).map_err(|_| APIError::Unauthorized);
        return Box::pin(async move {
            Ok(GeneralMiddleware {
                data: token_data.unwrap(),
            })
        });
    }
}

pub fn decode_token(jwt: &String) -> AppResult<TokenData<GeneralToken>> {
    let bytes_public_key = general_purpose::STANDARD
        .decode(dotenv!("GENERAL_TOKEN_PUBLIC_KEY"))
        .unwrap();
    let decoded_public_key = String::from_utf8(bytes_public_key).unwrap();
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[dotenv!("CLIENT_ID")]);
    jsonwebtoken::decode::<GeneralToken>(
        jwt,
        &DecodingKey::from_rsa_pem(decoded_public_key.as_bytes()).unwrap(),
        &validation,
    )
    .map_err(|_| APIError::Unauthorized)
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
