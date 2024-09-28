use std::future::Future;
use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use base64::engine::general_purpose;
use base64::Engine;
use dotenv_codegen::dotenv;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use uuid::Uuid;

use crate::{
    core::{
        config::{db::init_db, di::DiContainer},
        constants::AUTHORIZATION,
        error::APIError,
        types::AppResult,
    },
    features::{
        auth::{
            data::models::auth_token::AuthToken,
            domain::repository::auth_repository::AuthRepositoryImpl,
        },
        user::domain::{
            entity::user_response::UserResponse, repository::user_repository::UserRepositoryImpl,
        },
    },
    utils::token_helper::{is_auth_header_valid, token_extractor},
};

pub struct AuthMiddleware {
    pub user: UserResponse,
    pub login_session: Uuid,
}

impl FromRequest for AuthMiddleware {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output=Result<AuthMiddleware, Self::Error>>>>;

    // act as auth middleware
    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = init_db();
        let di = DiContainer::new(&pool);

        // clone the request headers to avoids lifetime issues
        let auth_header = request
            .headers()
            .get(AUTHORIZATION)
            .cloned()
            .expect("Authorization headers must be provided");

        Box::pin(async move {
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
            let token_data =
                decode_token_auth(&token.to_string()).map_err(|_| APIError::Unauthorized)?;

            let user_id = di.auth_repository.verify_token(&token_data).map_err(|_| {
                APIError::UnauthorizedMessage {
                    message: "The provided token has been revoked.".to_string(),
                }
            })?;

            let user = di.user_repository.find_user_by_id(user_id).map_err(|_| {
                APIError::UnauthorizedMessage {
                    message: "User not found".to_string(),
                }
            })?;

            Ok(AuthMiddleware {
                user,
                login_session: token_data.claims.login_session,
            })
        })
    }
}

pub fn decode_token_auth(jwt: &str) -> AppResult<TokenData<AuthToken>> {
    let bytes_public_key = general_purpose::STANDARD
        .decode(dotenv!("ACCESS_TOKEN_PUBLIC_KEY"))
        .unwrap();
    let decoded_public_key = String::from_utf8(bytes_public_key).unwrap();
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[dotenv!("CLIENT_ID")]);
    jsonwebtoken::decode::<AuthToken>(
        jwt,
        &DecodingKey::from_rsa_pem(decoded_public_key.as_bytes()).unwrap(),
        &validation,
    )
        .map_err(|_e| APIError::Unauthorized)
}
