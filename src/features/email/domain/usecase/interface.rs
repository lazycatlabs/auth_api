use crate::core::types::AppResult;
use crate::features::email::domain::usecase::dto::EmailParams;

pub trait IEmailService: Send + Sync {
    fn send_mail(&self, params: EmailParams) -> AppResult<String>;
}