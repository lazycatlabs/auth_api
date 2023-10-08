use std::sync::Arc;

use async_trait::async_trait;
use jsonwebtoken::TokenData;
use uuid::Uuid;
use validator::Validate;

use crate::core::error::APIError;
use crate::core::types::AppResult;
use crate::features::auth::data::models::auth_token::AuthToken;
use crate::features::auth::domain::{
    entity::auth::AuthEntity,
    repository::auth::IAuthRepository,
    usecase::{
        dto::LoginParams,
        interface::IAuthService,
    },
};
use crate::features::user::domain::repository::user::IUserRepository;

#[derive(Clone)]
pub struct AuthService

{
    pub auth_repo: Arc<dyn IAuthRepository>,
    pub user_repo: Arc<dyn IUserRepository>,
}

impl AuthService {
    pub fn new(
        auth_repo: Arc<dyn IAuthRepository>,
        user_repo: Arc<dyn IUserRepository>,
    ) -> Self {
        Self {
            auth_repo,
            user_repo,
        }
    }
}

#[async_trait]
impl IAuthService for AuthService
{
    async fn login(&self, params: LoginParams) -> AppResult<AuthEntity> {
        match params.validate() {
            Ok(_) => {
                let result = self.auth_repo.login(params).await?;
                Ok(result)
            }
            Err(e) => Err(APIError::BadRequest { message: e.to_string() })
        }
    }

    async fn logout(&self) -> AppResult<()> {
        todo!()
    }

    fn verify_token(&self, params: &TokenData<AuthToken>) -> AppResult<Uuid> {
        if self.auth_repo.is_valid_login_session(&params.claims) {
            Ok(params.claims.jti)
        } else {
            Err(APIError::Unauthorized)
        }
    }
}