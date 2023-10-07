use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    core::types::AppResult,
    features::{
        user::{
            data::models::user::User,
            domain::usecase::dto::*,
        }
    },
};

#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn create(&self, params: RegisterParams) -> AppResult<String>;
    // async fn update(&self, id: Uuid, params: &UpdateUserParams) -> AppResult<()>;
    async fn find_user_by_id(&self, id: Uuid) -> AppResult<User>;
    // async fn delete(&self, id: Uuid) -> AppResult<()>;
}

