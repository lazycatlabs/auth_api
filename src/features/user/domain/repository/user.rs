use uuid::Uuid;

use crate::{
    core::types::AppResult,
    features::user::domain::{
        entity::user::UserEntity, entity::user::UsersEntity, usecase::dto::*,
    },
};

pub trait IUserRepository: Send + Sync {
    fn create(&self, params: RegisterParams) -> AppResult<UserEntity>;
    fn find_user_by_id(&self, user_id: Uuid) -> AppResult<UserEntity>;
    fn update_user(&self, user_id: Uuid, params: UpdateUserParams) -> AppResult<UserEntity>;
    fn delete(&self, user_id: Uuid) -> AppResult<String>;
    fn users(&self, params: PaginationParams) -> AppResult<UsersEntity>;
}
