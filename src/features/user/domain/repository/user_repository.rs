use uuid::Uuid;

use crate::{
    core::types::AppResult,
    features::user::domain::{
        entity::user_response::{UserResponse, UsersResponse},
        usecase::{
            list_user::PaginationParams, register::RegisterParams, update_user::UpdateUserParams,
        },
    },
};

pub trait UserRepositoryImpl: Send + Sync {
    fn create(&self, params: RegisterParams) -> AppResult<UserResponse>;
    fn find_user_by_id(&self, user_id: Uuid) -> AppResult<UserResponse>;
    fn update_user(&self, user_id: Uuid, params: UpdateUserParams) -> AppResult<UserResponse>;
    fn delete(&self, user_id: Uuid) -> AppResult<String>;
    fn users(&self, params: PaginationParams) -> AppResult<UsersResponse>;
}
