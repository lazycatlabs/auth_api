use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    core::types::AppResult,
    features::user::domain::usecase::dto::*,
};
use crate::features::user::domain::entity::user::UserEntity;

#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn create(&self, params: RegisterParams) -> AppResult<String>;
    // async fn update(&self, id: Uuid, params: &UpdateUserParams) -> AppResult<()>;
    fn find_user_by_id(&self, id: &Uuid) -> AppResult<UserEntity>;
    // async fn delete(&self, id: Uuid) -> AppResult<()>;
}

