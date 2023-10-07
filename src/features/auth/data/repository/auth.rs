use async_trait::async_trait;
use bcrypt::verify;
use chrono::Utc;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    core::{
        config::db::PostgresDatabase, error::APIError,
        types::AppResult,
    },
    features::{
        auth::{
            domain::{
                entity::auth::AuthEntity,
                repository::auth::IAuthRepository,
                usecase::dto::{LoginHistoryParams, LoginParams},
            },
            data::models::{
                auth_token::AuthToken,
                login_info::LoginInfo
            }
        },
        user::{
            data::models::user::User,
            domain::repository::user::IUserRepository,
        },
    },
    schema::{
        login_history::{self},
        users::{self, dsl::*},
    },
};

pub struct AuthRepository<'auth, 'user> {
    source: &'auth PostgresDatabase,
    user: &'user dyn IUserRepository,
}

impl<'auth, 'user> AuthRepository<'auth, 'user> {
    pub fn new(
        source: &'auth PostgresDatabase,
        user: &'user dyn IUserRepository,
    ) -> Self {
        AuthRepository {
            source,
            user,
        }
    }
}

#[async_trait]
impl<'auth, 'user> IAuthRepository for AuthRepository<'auth, 'user> {
    async fn save_login_history(&self, params: Uuid) -> AppResult<usize> {
        // get user information by id
        if let Ok(user) = self.user.find_user_by_id(params).await {
            // save login history
            let now = Utc::now().naive_utc();
            let login_history_params = LoginHistoryParams { user_id: user.id, login_timestamp: now };
            return if let Ok(data) = diesel::insert_into(login_history::table)
                .values(&login_history_params)
                .execute(&mut self.source.pool.get().unwrap()) {
                Ok(data)
            } else {
                Err(APIError::InternalError)
            };
        }
        Err(APIError::InvalidCredentials)
    }

    async fn update_login_session(&self, params: Uuid, login_session_str: &str) -> bool {
        if let Ok(user) = self.user.find_user_by_id(params).await {
            diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str.to_string()))
                .execute(&mut self.source.pool.get().unwrap())
                .is_ok()
        } else {
            false
        }
    }

    async fn login(&self, params: LoginParams) -> AppResult<AuthEntity> {
        if let Ok(user) = users::table
            .filter(email.eq(&params.email))
            .get_result::<User>(&mut self.source.pool.get().unwrap())
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
                ).await {
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
}