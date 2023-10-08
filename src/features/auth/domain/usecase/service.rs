use async_trait::async_trait;
use validator::Validate;

use crate::core::error::APIError;
use crate::core::types::AppResult;
use crate::features::auth::domain::{
    entity::auth::AuthEntity,
    repository::auth::IAuthRepository,
    usecase::{
        dto::LoginParams,
        interface::IAuthService,
    },
};

#[derive(Clone)]
pub struct AuthService<T>
    where T: IAuthRepository
{
    pub repository: T,
}

impl<T> AuthService<T>
    where T: IAuthRepository
{
    pub fn new(repository: T) -> Self {
        Self {
            repository,
        }
    }
}

#[async_trait]
impl<T> IAuthService for AuthService<T>
    where T: IAuthRepository
{
    async fn login(&self, params: LoginParams) -> AppResult<AuthEntity> {
        match params.validate() {
            Ok(_) => {
                let result = self.repository.login(params).await?;
                Ok(result)
            }
            Err(e) => Err(APIError::BadRequest { message: e.to_string() })
        }
    }
}