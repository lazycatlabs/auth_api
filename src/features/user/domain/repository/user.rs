use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    core::types::AppResult,
    features::user::domain::{
        entity::user::UserEntity,
        usecase::dto::*,
    },
};

#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn create(&self, params: RegisterParams) -> AppResult<String>;
    fn find_user_by_id(&self, user_id: Uuid) -> AppResult<UserEntity>;
    fn update_user(&self, user_id: Uuid, params: UpdateUserParams) -> AppResult<UserEntity>;
    // async fn delete(&self, id: Uuid) -> AppResult<()>;
}

