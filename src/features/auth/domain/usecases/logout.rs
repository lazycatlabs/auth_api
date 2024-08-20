use uuid::Uuid;

use crate::{
    core::{error::APIError, types::AppResult},
    features::auth::{
        data::repository::auth_repository_impl::AuthRepository,
        domain::repository::auth_repository::AuthRepositoryImpl,
    },
};

pub fn logout(
    auth_repository: &AuthRepository,
    user_id: Uuid,
    login_session: Uuid,
) -> AppResult<()> {
    auth_repository
        .remove_user_session(user_id, login_session)
        .then_some(())
        .ok_or(APIError::InvalidCredentials)
}
