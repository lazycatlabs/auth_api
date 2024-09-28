use std::future::Future;
use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use base64::engine::general_purpose;
use base64::Engine;
use dotenv_codegen::dotenv;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};

use crate::{
    core::{constants::AUTHORIZATION, error::APIError, middlewares::auth::decode_token_auth, types::AppResult},
    features::auth::data::models::general_token::GeneralToken,
    utils::token_helper::{is_auth_header_valid, token_extractor},
};

#[allow(dead_code)]
pub struct GeneralMiddleware {
    pub data: TokenData<GeneralToken>,
}

impl FromRequest for GeneralMiddleware {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output=Result<GeneralMiddleware, Self::Error>>>>;

    // act as auth middleware
    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        // clone the request headers to avoids lifetime issues
        let auth_header = request.headers().get(AUTHORIZATION).cloned();

        Box::pin(async move {
            let auth_header = auth_header.ok_or_else(|| APIError::UnauthorizedMessage {
                message: "Authorization header not found".to_string(),
            })?;

            if !is_auth_header_valid(&auth_header) {
                return Err(APIError::UnauthorizedMessage {
                    message: "Invalid authorization headers".to_string(),
                });
            }

            let auth_str = auth_header
                .to_str()
                .map_err(|_| APIError::UnauthorizedMessage {
                    message: "Invalid authorization headers".to_string(),
                })?;

            let token = token_extractor(auth_str);
            // check general token first
            let _ = decode_token(&token).map_err(|_|
                decode_token_auth(&token).map_err(|_| APIError::Unauthorized));

            let mock_data = TokenData {
                header: Default::default(),
                claims: GeneralToken {
                    // Add fields of GeneralToken here
                    aud: "".to_string(),
                    exp: 0,
                    iat: 0,
                },

            };
            Ok(GeneralMiddleware { data: mock_data })
        })
    }
}

pub fn decode_token(jwt: &str) -> AppResult<TokenData<GeneralToken>> {
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
