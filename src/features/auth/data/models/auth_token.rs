use base64::{engine::general_purpose, Engine};
use chrono::{Duration, Utc};
use dotenv_codegen::dotenv;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use uuid::Uuid;

use crate::{
    camel_case_struct,
    core::{error::APIError, types::AppResult},
    features::auth::data::models::login_info::LoginInfo,
};

camel_case_struct!(AuthToken {
    jti: Uuid,
    aud: String,
    iat: i64,
    exp: i64,
    login_session: Uuid
});

impl AuthToken {
    pub fn generate_token(login: &LoginInfo) -> AppResult<String> {
        let bytes_private_key = general_purpose::STANDARD
            .decode(dotenv!("ACCESS_TOKEN_PRIVATE_KEY"))
            .unwrap();
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
        )
        .map_err(|_| APIError::InternalError)
    }
}
