use crate::{
    core::types::AppResult,
    features::auth::data::models::login_history::LoginHistory,
};
use crate::features::email::domain::usecase::dto::EmailParams;

pub trait IEmailRepository: Send + Sync {
    fn send_mail(&self, email_params: EmailParams) -> AppResult<LoginHistory>;
}

