use actix_web::{HttpRequest, web};
use actix_web::http::header::HeaderValue;
use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use dotenv_codegen::dotenv;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::db::Pool;
use crate::error::ServiceError;
use crate::models::user::{LoginInfoDTO, User};

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub jti: Uuid,
    // audience
    pub aud: String,
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    pub login_session: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JWTResponse {
    pub token: String,
    pub token_type: String,
}

impl JWTResponse {
    pub fn new(token: String) -> Self {
        Self {
            token: format!("{}.{}", dotenv!("TOKEN_PREFIX"), token),
            token_type: "Bearer".to_string(),
        }
    }
}

impl UserToken {
    pub fn generate_token(login: &LoginInfoDTO) -> Result<String, ServiceError> {
        let bytes_private_key = general_purpose::STANDARD.decode(dotenv!("ACCESS_TOKEN_PRIVATE_KEY")).unwrap();
        let decoded_private_key = String::from_utf8(bytes_private_key).unwrap();
        let now = Utc::now().timestamp();
        let exp = now + 1000 * 60 * 60 * 24 * 7; // 7 days
        let payload = UserToken {
            jti: login.id.parse().unwrap(),
            aud: dotenv!("CLIENT_ID").to_string(),
            iat: now,
            exp,
            login_session: login.login_session.clone(),
        };
        jsonwebtoken::encode(
            &Header::new(Algorithm::RS256),
            &payload,
            &EncodingKey::from_rsa_pem(decoded_private_key.as_bytes()).unwrap(),
        ).map_err(|_e| ServiceError::InternalError)
    }

    pub fn decode_token(jwt: &String) -> Result<TokenData<UserToken>, ServiceError> {
        let bytes_public_key = general_purpose::STANDARD.decode(dotenv!("ACCESS_TOKEN_PUBLIC_KEY")).unwrap();
        let decoded_public_key = String::from_utf8(bytes_public_key).unwrap();
        jsonwebtoken::decode::<UserToken>(
            jwt,
            &DecodingKey::from_rsa_pem(decoded_public_key.as_bytes()).unwrap(),
            &Validation::new(Algorithm::RS256),
        ).map_err(|_e| ServiceError::Unauthorized)
    }

    pub fn verify_token(jwt: &TokenData<UserToken>, pool: &web::Data<Pool>) -> Result<Uuid, String> {
        if User::is_valid_login_session(&jwt.claims, &mut pool.get().unwrap()) {
            Ok(jwt.claims.jti)
        } else {
            Err("Invalid token".to_string())
        }
    }

    pub fn is_auth_header_valid(auth_header: &HeaderValue) -> bool {
        if let Ok(auth_str) = auth_header.to_str() {
            return auth_str.starts_with("bearer") || auth_str.starts_with("Bearer");
        }
        false
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

// impl FromRequest for UserToken {
//     type Error = ServiceError;
//
//     type Future = Pin<Box<dyn Future<Output=Result<UserToken, Self::Error>>>>;
//
//     fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
//         if let Ok(jwt) = UserToken::parse_jwt_from_request(request) {
//             if let Ok(user_token) = UserToken::verify_token(&jwt) {
//                 println!("TOKEN {}", user_token);
//                 return Box::pin(async move { Ok(user_token) });
//             }
//         }
//         Box::pin(async move { Err(ServiceError::Unauthorized) })
//     }
// }
//

