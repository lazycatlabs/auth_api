use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;
use validator::Validate;

use crate::core::error::APIError;
use crate::core::types::AppResult;
use crate::features::user::domain::{
    repository::user::IUserRepository,
    usecase::{
        dto::RegisterParams,
        interface::IUserService,
    },
};
use crate::features::user::domain::entity::user::UserEntity;

#[derive(Clone)]
pub struct UserService
{
    pub user_repo: Arc<dyn IUserRepository>,
}

impl UserService
{
    pub fn new(user_repo: Arc<dyn IUserRepository>) -> Self {
        Self {
            user_repo,
        }
    }
}

#[async_trait]
impl IUserService for UserService
{
    async fn register(&self, params: RegisterParams) -> AppResult<String> {
        match params.validate() {
            Ok(_) => {
                let result = self.user_repo.create(params).await?;
                Ok(result)
            }
            Err(e) => Err(APIError::BadRequest { message: e.to_string() })
        }
    }

    fn find_user_by_id(&self, id: &Uuid) -> AppResult<UserEntity> {
        match self.user_repo.find_user_by_id(id) {
            Ok(data) => Ok(data),
            Err(e) => Err(e)
        }
    }
}