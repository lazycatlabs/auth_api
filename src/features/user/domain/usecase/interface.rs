use async_trait::async_trait;

use crate::core::types::AppResult;
use crate::features::user::domain::usecase::dto::RegisterParams;

#[async_trait]
pub trait IUserService: Send + Sync {
    async fn register(&self, params: RegisterParams) -> AppResult<String>;
}