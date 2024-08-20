use jsonwebtoken::TokenData;
use uuid::Uuid;

use crate::{
    core::types::AppResult,
    features::auth::{
        data::models::{auth_token::AuthToken, login_history::LoginHistory},
        domain::usecases::dto::*,
    },
};

pub trait IAuthService: Send + Sync {
    // fn login(&self, params: LoginParams) -> AppResult<AuthResponse>;
    // fn logout(&self, user: Uuid, login_session: Uuid) -> AppResult<()>;
    fn verify_token(&self, params: &TokenData<AuthToken>) -> AppResult<Uuid>;
    fn login_session(&self, user: Uuid) -> AppResult<Vec<LoginHistory>>;
    // fn general_token(&self, token: GeneralTokenParams) -> AppResult<AuthResponse>;
    fn update_password(&self, user: Uuid, params: UpdatePasswordParams) -> AppResult<()>;
}
