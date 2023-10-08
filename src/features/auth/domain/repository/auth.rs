use async_trait::async_trait;
use uuid::Uuid;

use crate::core::types::AppResult;
use crate::features::auth::{
    data::models::auth_token::AuthToken,
    domain::{
        entity::auth::AuthEntity,
        usecase::dto::LoginParams,
    },
};

#[async_trait]
pub trait IAuthRepository: Send + Sync {
    async fn save_login_history(&self, params: Uuid) -> AppResult<usize>;
    fn update_login_session(&self, params: Uuid, login_session_str: &str) -> bool;
    async fn login(&self, params: LoginParams) -> AppResult<AuthEntity>;
    fn is_valid_login_session(&self, params: &AuthToken) -> bool;

    // async fn update(&self, id: Uuid, params: &UpdateUserParams) -> AppResult<()>;
    // async fn find_by_id(&self, id: Uuid) -> AppResult<User>;
    // async fn delete(&self, id: Uuid) -> AppResult<()>;
}

