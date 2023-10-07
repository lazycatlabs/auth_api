// use base64::{Engine as _, engine::general_purpose};
// use chrono::Utc;
// use dotenv_codegen::dotenv;
// use jsonwebtoken::{Algorithm, EncodingKey, Header};
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;
//
// use crate::{
//     models::user::LoginInfoDTO,
// };
// use crate::core::error::APIError;
//
// #[derive(Serialize, Deserialize)]
// pub struct UserToken {
//     pub jti: Uuid,
//     // audience
//     pub aud: String,
//     // issued at
//     pub iat: i64,
//     // expiration
//     pub exp: i64,
//     pub login_session: String,
// }
//
// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct JWTResponse {
//     pub token: String,
//     pub token_type: String,
// }
//
// impl JWTResponse {
//     pub fn new(token: String) -> Self {
//         Self {
//             token: format!("{}.{}", dotenv!("TOKEN_PREFIX"), token),
//             token_type: "Bearer".to_string(),
//         }
//     }
// }
//
// impl UserToken {
//     pub fn generate_token(login: &LoginInfoDTO) -> Result<String, APIError> {
//         let bytes_private_key = general_purpose::STANDARD.decode(dotenv!("ACCESS_TOKEN_PRIVATE_KEY")).unwrap();
//         let decoded_private_key = String::from_utf8(bytes_private_key).unwrap();
//         let now = Utc::now().timestamp();
//         let exp = now + 1000 * 60 * 60 * 24 * 7; // 7 days
//         let payload = UserToken {
//             jti: login.id.parse().unwrap(),
//             aud: dotenv!("CLIENT_ID").to_string(),
//             iat: now,
//             exp,
//             login_session: login.login_session.clone(),
//         };
//         jsonwebtoken::encode(
//             &Header::new(Algorithm::RS256),
//             &payload,
//             &EncodingKey::from_rsa_pem(decoded_private_key.as_bytes()).unwrap(),
//         ).map_err(|_e| APIError::InternalError)
//     }
// }