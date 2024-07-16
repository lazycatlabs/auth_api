use std::sync::Arc;

use jsonwebtoken::TokenData;
use uuid::Uuid;
use validator::Validate;

use crate::{
    core::{error::APIError, types::AppResult},
    features::{
        auth::{
            data::models::{auth_token::AuthToken, login_history::LoginHistory},
            domain::{
                entity::auth::AuthEntity,
                repository::auth::IAuthRepository,
                usecase::{dto::*, interface::IAuthService},
            },
        },
        user::domain::repository::user::IUserRepository,
    },
};

#[derive(Clone)]
pub struct AuthService {
    pub auth_repo: Arc<dyn IAuthRepository>,
    pub user_repo: Arc<dyn IUserRepository>,
}

impl AuthService {
    pub fn new(auth_repo: Arc<dyn IAuthRepository>, user_repo: Arc<dyn IUserRepository>) -> Self {
        Self {
            auth_repo,
            user_repo,
        }
    }
}

impl IAuthService for AuthService {
    fn login(&self, params: LoginParams) -> AppResult<AuthEntity> {
        params
            .validate()
            .map_err(|e| APIError::BadRequest {
                message: e.to_string(),
            })
            .and_then(|_| self.auth_repo.login(params))
    }

    fn logout(&self, user: Uuid, login_session: Uuid) -> AppResult<()> {
        self.auth_repo
            .remove_user_session(user, login_session)
            .then_some(())
            .ok_or(APIError::InvalidCredentials)
    }

    fn verify_token(&self, params: &TokenData<AuthToken>) -> AppResult<Uuid> {
        self.auth_repo
            .is_valid_login_session(params.claims.jti, params.claims.login_session)
            .then(|| params.claims.jti)
            .ok_or(APIError::Unauthorized)
    }

    fn login_session(&self, user: Uuid) -> AppResult<Vec<LoginHistory>> {
        self.auth_repo.get_user_session(user)
    }

    fn general_token(&self, token: GeneralTokenParams) -> AppResult<AuthEntity> {
        self.auth_repo.general_token(token)
    }

    fn update_password(&self, user: Uuid, params: UpdatePasswordParams) -> AppResult<()> {
        params
            .validate()
            .map_err(|e| APIError::BadRequest {
                message: e.to_string(),
            })
            .and_then(|_| {
                (params.old_password != params.new_password)
                    .then(|| self.auth_repo.update_password(user, params))
                    .ok_or(APIError::BadRequest {
                        message: "Old password and new password must be different".to_string(),
                    })
            })?
    }
}
