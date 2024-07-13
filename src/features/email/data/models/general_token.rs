use base64::{Engine, engine::general_purpose};
use chrono::{Duration, Utc};
use dotenv_codegen::dotenv;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        error::APIError,
        types::AppResult,
    },
};

#[derive(Serialize, Deserialize,Debug)]
pub struct GeneralToken {
    // audience
    pub aud: String,
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
}

impl GeneralToken {
    pub fn generate_general_token() -> AppResult<String> {
        let bytes_private_key = general_purpose::STANDARD.decode(dotenv!("GENERAL_TOKEN_PRIVATE_KEY")).unwrap();
        let decoded_private_key = String::from_utf8(bytes_private_key).unwrap();
        let now = Utc::now();
        let exp = now + Duration::days(7); // 7 days
        let payload = GeneralToken {
            aud: dotenv!("CLIENT_ID").to_string(),
            iat: now.timestamp(),
            exp:exp.timestamp(),
        };
        jsonwebtoken::encode(
            &Header::new(Algorithm::RS256),
            &payload,
            &EncodingKey::from_rsa_pem(decoded_private_key.as_bytes()).unwrap(),
        ).map_err(|_e| APIError::InternalError)
    }
}