use async_trait::async_trait;
use uuid::Uuid;

use crate::core::types::AppResult;
use crate::features::user::domain::entity::user::UserEntity;
use crate::features::user::domain::usecase::dto::RegisterParams;

#[async_trait]
pub trait IUserService: Send + Sync {
    async fn register(&self, params: RegisterParams) -> AppResult<String>;

    fn find_user_by_id(&self, id: &Uuid) -> AppResult<UserEntity>;
}