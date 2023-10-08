use async_trait::async_trait;
use bcrypt::verify;
use chrono::Utc;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    core::{
        error::APIError,
        types::{AppResult, DBConn},
    },
    features::{
        auth::{
            data::models::{
                auth_token::AuthToken,
                login_info::LoginInfo,
            },
            domain::{
                entity::auth::AuthEntity,
                repository::auth::IAuthRepository,
                usecase::dto::{LoginHistoryParams, LoginParams},
            },
        },
        user::data::models::user::User,
    },
    schema::{
        login_history::{self},
        users::{self, dsl::*},
    },
};

#[derive(Clone)]
pub struct AuthRepository {
    source: DBConn,
}

impl AuthRepository {
    pub fn new(
        source: DBConn,
    ) -> Self {
        AuthRepository {
            source,
        }
    }
}

#[async_trait]
impl IAuthRepository for AuthRepository {
    async fn save_login_history(&self, params: Uuid) -> AppResult<usize> {
        // get user information by id
        let now = Utc::now().naive_utc();
        let login_history_params = LoginHistoryParams { user_id: params, login_timestamp: now };
        return if let Ok(data) = diesel::insert_into(login_history::table)
            .values(&login_history_params)
            .execute(&mut self.source.get().unwrap()) {
            Ok(data)
        } else {
            Err(APIError::InternalError)
        };
    }

    fn update_login_session(&self, params: Uuid, login_session_str: &str) -> bool {
        diesel::update(users.find(params))
            .set(login_session.eq(login_session_str.to_string()))
            .execute(&mut self.source.get().unwrap())
            .is_ok()
    }

    async fn login(&self, params: LoginParams) -> AppResult<AuthEntity> {
        if let Ok(user) = users::table
            .filter(email.eq(&params.email))
            .get_result::<User>(&mut self.source.get().unwrap())
        {
            if !user.password.is_empty()
                && verify(&params.password, &user.password).unwrap() {
                if self.save_login_history(user.id).await.is_err() {
                    return Err(APIError::InternalError);
                }
                let login_session_str = User::generate_login_session();
                if self.update_login_session(
                    user.id,
                    login_session_str.as_str(),
                ) {
                    let login_info = LoginInfo {
                        id: user.id.to_string(),
                        email: user.email,
                        login_session: login_session_str,
                    };

                    let generate_token = AuthToken::generate_token(&login_info);
                    return match generate_token {
                        Ok(token) => {
                            if login_info.login_session.is_empty() {
                                Err(APIError::Unauthorized)
                            } else {
                                Ok(AuthEntity::new(token))
                            }
                        }
                        Err(e) => { Err(e) }
                    };
                }
            }
            return Err(APIError::InvalidCredentials);
        }

        Err(APIError::InvalidCredentials)
    }

    fn is_valid_login_session(&self, params: &AuthToken) -> bool {
        users
            .filter(id.eq(&params.jti))
            .filter(login_session.eq(&params.login_session))
            .get_result::<User>(&mut self.source.get().unwrap())
            .is_ok()
    }
}