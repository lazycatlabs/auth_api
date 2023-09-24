use std::pin::Pin;

use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use chrono::Utc;
use dotenv_codegen::dotenv;
use futures_util::Future;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::user::ServiceError;

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub id: Uuid,
    pub email: String,
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct JWTResponse {
    pub token: String,
    pub token_type: String,
}

impl JWTResponse {
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type: "bearer".to_string(),
        }
    }
}

impl UserToken {
    pub fn generate_token(user_id: &Uuid, email: &String) -> Result<String, ServiceError> {
        let now = Utc::now().timestamp();
        let exp = now + 1000 * 60 * 60 * 24 * 7; // 7 days
        let payload = UserToken {
            id: user_id.clone(),
            email: email.clone(),
            iat: now,
            exp,
        };
        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(dotenv!("JWT_SECRET").as_ref()),
        ).map_err(|_e| ServiceError::InternalError)
    }

    pub fn decode_token(jwt: &String) -> Result<Self, ServiceError> {
        jsonwebtoken::decode::<UserToken>(
            jwt,
            &DecodingKey::from_secret(dotenv!("JWT_SECRET").as_ref()),
            &Validation::new(Algorithm::HS256),
        )
            .and_then(|token| Ok(token.claims))
            .map_err(|_e| ServiceError::Unauthorized)
    }

    pub fn verify_token(jwt: &String) -> Result<Self, ServiceError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = false;

        jsonwebtoken::decode::<UserToken>(
            jwt,
            &DecodingKey::from_secret(dotenv!("JWT_SECRET").as_ref()), &validation)
            .and_then(|token| Ok(token.claims))
            .map_err(|_e| ServiceError::Unauthorized)
    }

    pub fn parse_jwt_from_request(request: &HttpRequest) -> Result<String, ServiceError> {
        // Validate if Auth data format is correct
        if let Some(auth_header) = request.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if !(auth_str.starts_with("bearer") || auth_str.starts_with("Bearer")) {
                    return Err(ServiceError::Unauthorized);
                }

                let bearer_token: Vec<&str> = auth_str.split_whitespace().collect();
                if bearer_token.len() != 2 {
                    return Err(ServiceError::Unauthorized);
                }

                let token = bearer_token[1];
                return Ok(token.to_owned());
            }
        }

        Err(ServiceError::Unauthorized)
    }
}

impl FromRequest for UserToken {
    type Error = ServiceError;

    type Future = Pin<Box<dyn Future<Output=Result<UserToken, Self::Error>>>>;

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Ok(jwt) = UserToken::parse_jwt_from_request(request) {
            if let Ok(user_token) = UserToken::verify_token(&jwt) {
                println!("TOKEN {}", user_token.id);
                return Box::pin(async move { Ok(user_token) });
            }
        }
        Box::pin(async move { Err(ServiceError::Unauthorized) })
    }
}


