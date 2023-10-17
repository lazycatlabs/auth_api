use async_trait::async_trait;
use jsonwebtoken::TokenData;
use uuid::Uuid;

use crate::{
    core::types::AppResult,
    features::auth::{
        data::models::auth_token::AuthToken,
        domain::{
            entity::auth::AuthEntity,
            usecase::dto::LoginParams,
        },
    },
};

#[async_trait]
pub trait IAuthService: Send + Sync {
    async fn login(&self, params: LoginParams) -> AppResult<AuthEntity>;
    fn logout(&self, user: Uuid, login_session: Uuid) -> AppResult<()>;
    fn verify_token(&self, params: &TokenData<AuthToken>) -> AppResult<Uuid>;
}