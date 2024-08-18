use uuid::Uuid;

use crate::{
    core::types::AppResult,
    features::user::domain::{
        entity::user_response::{UserEntity, UsersEntity},
        usecase::dto::{PaginationParams, RegisterParams, UpdateUserParams},
    },
};

pub trait IUserService: Send + Sync {
    fn register(&self, params: RegisterParams) -> AppResult<UserEntity>;
    fn find_user_by_id(&self, user_id: Uuid) -> AppResult<UserEntity>;
    fn update_user(&self, user_id: Uuid, params: UpdateUserParams) -> AppResult<UserEntity>;
    fn delete_user(&self, user_id: Uuid) -> AppResult<String>;
    fn users(&self, params: PaginationParams) -> AppResult<UsersEntity>;
}
