use crate::{
    core::types::AppResult,
    features::user::{
        data::repository::user_repository_impl::UserRepository,
        domain::{
            entity::user_response::UsersResponse, repository::user_repository::UserRepositoryImpl,
        },
    },
};

use crate::camel_case_struct;

camel_case_struct!(PaginationParams {
     page: Option<i64>,
     per_page: Option<i64>,
});

pub fn list_user(
    user_repository: &UserRepository,
    params: PaginationParams,
) -> AppResult<UsersResponse> {
    user_repository.users(params)
}
