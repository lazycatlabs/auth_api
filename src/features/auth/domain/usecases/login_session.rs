use uuid::Uuid;

use crate::{
    core::types::AppResult,
    features::auth::{
        data::{
            models::login_history::LoginHistory, repository::auth_repository_impl::AuthRepository,
        },
        domain::repository::auth_repository::AuthRepositoryImpl,
    },
};

pub fn login_session(
    auth_repository: &AuthRepository,
    user_id: Uuid,
) -> AppResult<Vec<LoginHistory>> {
    auth_repository.get_user_session(user_id)
}
