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
use crate::features::auth::data::models::login_history::LoginHistory;

#[async_trait]
pub trait IAuthService: Send + Sync {
    fn login(&self, params: LoginParams) -> AppResult<AuthEntity>;
    fn logout(&self, user: Uuid, login_session: Uuid) -> AppResult<()>;
    fn verify_token(&self, params: &TokenData<AuthToken>) -> AppResult<Uuid>;
    fn login_session(&self, user: Uuid) -> AppResult<Vec<LoginHistory>>;
}