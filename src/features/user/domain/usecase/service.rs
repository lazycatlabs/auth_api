use std::sync::Arc;

use crate::features::user::domain::{
    repository::user_repository::UserRepositoryImpl, usecase::interface::IUserService,
};

#[derive(Clone)]
pub struct UserService {
    pub user_repo: Arc<dyn UserRepositoryImpl>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepositoryImpl>) -> Self {
        Self { user_repo }
    }
}

impl IUserService for UserService {
    // fn register(&self, params: RegisterParams) -> AppResult<UserEntity> {
    //     params
    //         .validate()
    //         .map(|_| self.user_repo.create(params))
    //         .map_err(|e| APIError::BadRequest {
    //             message: e.to_string(),
    //         })?
    // }

    // fn find_user_by_id(&self, user_id: Uuid) -> AppResult<UserResponse> {
    //     self.user_repo.find_user_by_id(user_id)
    // }

    // fn update_user(&self, user_id: Uuid, params: UpdateUserParams) -> AppResult<UserResponse> {
    //     self.user_repo.update_user(user_id, params)
    // }

    // fn delete_user(&self, user_id: Uuid) -> AppResult<String> {
    //     self.user_repo.delete(user_id)
    // }

    // fn users(&self, params: PaginationParams) -> AppResult<UsersResponse> {
    //     self.user_repo.users(params)
    // }
}
