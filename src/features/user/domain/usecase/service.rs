use async_trait::async_trait;

use crate::core::types::AppResult;
use crate::features::user::domain::{
    repository::user::IUserRepository,
    usecase::{
        dto::RegisterParams,
        interface::IUserService,
    },
};

#[derive(Clone)]
pub struct UserService< T>
    where T: IUserRepository
{
    pub repository:  T,
}

impl< T> UserService< T>
    where T: IUserRepository
{
    pub fn new(repository:  T) -> Self {
        Self {
            repository,
        }
    }
}

#[async_trait]
impl< T> IUserService for UserService< T>
    where T: IUserRepository
{
    async fn register(&self, params: RegisterParams) -> AppResult<String> {
        let result = self.repository.create(params).await?;
        Ok(result)
    }
}