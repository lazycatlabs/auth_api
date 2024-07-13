use std::sync::Arc;

use crate::core::types::AppResult;
use crate::features::email::domain::repository::email::IEmailRepository;
use crate::features::email::domain::usecase::dto::EmailParams;
use crate::features::email::domain::usecase::interface::IEmailService;
use crate::features::user::domain::repository::user::IUserRepository;

#[derive(Clone)]
pub struct EmailService

{
    pub email_repo: Arc<dyn IEmailRepository>,
    pub user_repo: Arc<dyn IUserRepository>,
}

impl EmailService {
    pub fn new(
        email_repo: Arc<dyn IEmailRepository>,
        user_repo: Arc<dyn IUserRepository>,
    ) -> Self {
        Self {
            email_repo,
            user_repo,
        }
    }
}

impl IEmailService for EmailService
{
    fn send_mail(&self, params: EmailParams) -> AppResult<String> {
        todo!()
    }
    // fn login(&self, params: LoginParams) -> AppResult<AuthEntity> {
    //     match params.validate() {
    //         Ok(_) => self.auth_repo.login(params),
    //         Err(e) => Err(APIError::BadRequest { message: e.to_string() })
    //     }
    // }
}