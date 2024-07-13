use base64::{Engine, engine::general_purpose};
use chrono::{Duration, Utc};
use dotenv_codegen::dotenv;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    core::{
        error::APIError,
        types::AppResult,
    },
    features::auth::data::models::login_info::LoginInfo,
};

#[derive(Serialize, Deserialize)]
pub struct AuthToken {
    pub jti: Uuid,
    // audience
    pub aud: String,
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    pub login_session: Uuid,
}

impl AuthToken {
    pub fn generate_token(login: &LoginInfo) -> AppResult<String> {
        let bytes_private_key = general_purpose::STANDARD.decode(dotenv!("ACCESS_TOKEN_PRIVATE_KEY")).unwrap();
        let decoded_private_key = String::from_utf8(bytes_private_key).unwrap();
        let now = Utc::now();
        let exp = now + Duration::days(7); // 7 days
        let payload = AuthToken {
            jti: login.id.parse().unwrap(),
            aud: dotenv!("CLIENT_ID").to_string(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
            login_session: login.login_session.clone(),
        };
        jsonwebtoken::encode(
            &Header::new(Algorithm::RS256),
            &payload,
            &EncodingKey::from_rsa_pem(decoded_private_key.as_bytes()).unwrap(),
        ).map_err(|_e| APIError::InternalError)
    }
}