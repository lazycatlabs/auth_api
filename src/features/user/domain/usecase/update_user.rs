use uuid::Uuid;

use diesel::AsChangeset;
use serde::{Deserialize, Serialize};

use crate::{
    core::types::AppResult,
    features::user::{
        data::repository::user_repository_impl::UserRepository,
        domain::{
            entity::user_response::UserResponse, repository::user_repository::UserRepositoryImpl,
        },
    },
    schema::users,
};

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct UpdateUserParams {
    pub name: Option<String>,
    pub photo: Option<String>,
    pub verified: Option<bool>,
}

pub fn update_user(
    user_repository: &UserRepository,
    user_id: Uuid,
    params: UpdateUserParams,
) -> AppResult<UserResponse> {
    user_repository.update_user(user_id, params)
}
