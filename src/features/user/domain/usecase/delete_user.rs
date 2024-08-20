use uuid::Uuid;

use crate::{
    core::types::AppResult,
    features::user::{
        data::repository::user_repository_impl::UserRepository,
        domain::repository::user_repository::UserRepositoryImpl,
    },
};

pub fn delete_user(user_repository: &UserRepository, user_id: Uuid) -> AppResult<String> {
    user_repository.delete(user_id)
}
