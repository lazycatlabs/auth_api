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
                login_history::LoginHistory,
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
        login_history::{
            self,
            dsl::*,
            id,
            user_id,
        },
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
    async fn add_user_session(&self, user: Uuid, login_params: LoginParams) -> AppResult<LoginHistory> {
        // get user information by id
        let now = Utc::now().naive_utc();
        let login_history_params = LoginHistoryParams {
            user_id: user,
            ip_addr: login_params.ip_addr.unwrap(),
            os_info: login_params.os_info,
            device_info: login_params.device_info,
            login_timestamp: now,
        };
        return if let Ok(data) = diesel::insert_into(login_history::table)
            .values(&login_history_params)
            .get_result::<LoginHistory>(&mut self.source.get().unwrap()) {
            Ok(data)
        } else {
            Err(APIError::InternalError)
        };
    }


    async fn login(&self, params: LoginParams) -> AppResult<AuthEntity> {
        if let Ok(user) = users::table
            .filter(email.eq(&params.email))
            .get_result::<User>(&mut self.source.get().unwrap())
        {
            if !user.password.is_empty()
                && verify(&params.password, &user.password).unwrap() {
                return if let Ok(login_session) = self.add_user_session(user.id, params).await {
                    let login_info = LoginInfo {
                        id: user.id.to_string(),
                        email: user.email,
                        login_session: login_session.id,
                    };

                    let generate_token = AuthToken::generate_token(&login_info);
                    match generate_token {
                        Ok(token) => Ok(AuthEntity::new(token)),
                        Err(e) => { Err(e) }
                    }
                } else {
                    Err(APIError::InternalError)
                };
            }
            return Err(APIError::InvalidCredentials);
        }

        Err(APIError::InvalidCredentials)
    }

    fn is_valid_login_session(&self, params: &AuthToken) -> bool {
        login_history
            .filter(user_id.eq(&params.jti))
            .filter(id.eq(&params.login_session))
            .execute(&mut self.source.get().unwrap())
            .is_ok()
    }
}