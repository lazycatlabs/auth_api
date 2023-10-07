use async_trait::async_trait;

use crate::core::types::AppResult;
use crate::features::user::domain::{
    repository::user::IUserRepository,
    usecase::{
        dto::RegisterParams,
        interface::IUserService,
    },
};

pub struct UserService<'user, T>
    where T: IUserRepository
{
    pub repository: &'user T,
}

impl<'user, T> UserService<'user, T>
    where T: IUserRepository
{
    pub fn new(repository: &'user T) -> Self {
        Self {
            repository,
        }
    }
}

#[async_trait]
impl<'user, T> IUserService for UserService<'user, T>
    where T: IUserRepository
{
    async fn register(&self, params: RegisterParams) -> AppResult<String> {
        let result = self.repository.create(params).await?;
        Ok(result)
    }
}