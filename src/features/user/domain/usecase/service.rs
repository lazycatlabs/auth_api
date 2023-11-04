use std::sync::Arc;

use uuid::Uuid;
use validator::Validate;

use crate::{
    core::{
        error::APIError,
        types::AppResult,
    },
    features::user::domain::{
        entity::user::UserEntity,
        repository::user::IUserRepository,
        usecase::{
            dto::{
                RegisterParams,
                UpdateUserParams,
            },
            interface::IUserService,
        },
    },
};

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

impl IUserService for UserService
{
    fn register(&self, params: RegisterParams) -> AppResult<String> {
        match params.validate() {
            Ok(_) => self.user_repo.create(params),
            Err(e) => Err(APIError::BadRequest { message: e.to_string() })
        }
    }

    fn find_user_by_id(&self, user_id: Uuid) -> AppResult<UserEntity> {
        match self.user_repo.find_user_by_id(user_id) {
            Ok(data) => Ok(data),
            Err(e) => Err(e)
        }
    }

    fn update_user(&self, user_id: Uuid, params: UpdateUserParams) -> AppResult<UserEntity> {
        match self.user_repo.update_user(user_id, params) {
            Ok(data) => Ok(data),
            Err(e) => Err(e)
        }
    }

    fn delete_user(&self, user_id: Uuid) -> AppResult<String> {
        match self.user_repo.delete(user_id) {
            Ok(data) => Ok(data),
            Err(e) => Err(e)
        }
    }
}