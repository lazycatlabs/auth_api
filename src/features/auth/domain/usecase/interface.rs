use async_trait::async_trait;

use crate::core::types::AppResult;
use crate::features::auth::domain::{
    entity::auth::AuthEntity,
    usecase::dto::LoginParams,
};

#[async_trait]
pub trait IAuthService: Send + Sync {
    async fn login(&self, params: LoginParams) -> AppResult<AuthEntity>;
}