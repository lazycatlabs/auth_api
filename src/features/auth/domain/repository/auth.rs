use uuid::Uuid;

use crate::{
    core::types::AppResult,
    features::auth::{
        data::models::login_history::LoginHistory,
        domain::{entity::auth::AuthEntity, usecase::dto::*},
    },
};

pub trait IAuthRepository: Send + Sync {
    fn add_user_session(&self, user: Uuid, login_params: LoginParams) -> AppResult<LoginHistory>;
    fn remove_user_session(&self, user: Uuid, login_session: Uuid) -> bool;
    fn get_user_session(&self, user: Uuid) -> AppResult<Vec<LoginHistory>>;
    fn login(&self, params: LoginParams) -> AppResult<AuthEntity>;
    fn general_token(&self, params: GeneralTokenParams) -> AppResult<AuthEntity>;
    fn is_valid_login_session(&self, user: Uuid, login_session: Uuid) -> bool;
    fn update_password(&self, user: Uuid, params: UpdatePasswordParams) -> AppResult<()>;
}
