use bcrypt::{verify, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};
use uuid::Uuid;

use crate::{
    core::{
        error::APIError,
        types::{AppResult, DBConn},
    },
    features::{
        auth::{
            data::models::{
                auth_token::AuthToken, general_token::GeneralToken, login_history::LoginHistory,
                login_info::LoginInfo,
            },
            domain::{
                entity::auth::AuthEntity, repository::auth::IAuthRepository, usecase::dto::*,
            },
        },
        user::data::models::user::User,
    },
    schema::{
        login_history::{self, dsl::*, id as login_history_id, user_id as login_history_user_id},
        users::{self, dsl::*, id as user_id},
    },
};

#[derive(Clone)]
pub struct AuthRepository {
    source: DBConn,
}

impl AuthRepository {
    pub fn new(source: DBConn) -> Self {
        AuthRepository { source }
    }
}

impl IAuthRepository for AuthRepository {
    fn add_user_session(&self, user: Uuid, login_params: LoginParams) -> AppResult<LoginHistory> {
        // get user information by id
        let now = Utc::now().naive_utc();
        let login_history_params = LoginHistoryParams {
            user_id: user,
            ip_addr: login_params.ip_addr.unwrap(),
            os_info: login_params.os_info,
            device_info: login_params.device_info,
            login_timestamp: now,
            fcm_token: login_params.fcm_token,
        };
        return if let Ok(data) = diesel::insert_into(login_history::table)
            .values(&login_history_params)
            .get_result::<LoginHistory>(&mut self.source.get().unwrap())
        {
            Ok(data)
        } else {
            Err(APIError::InternalError)
        };
    }

    fn remove_user_session(&self, user: Uuid, login_session: Uuid) -> bool {
        if self.is_valid_login_session(user, login_session) {
            diesel::delete(login_history.filter(login_history_id.eq(login_session)))
                .execute(&mut self.source.get().unwrap())
                .expect("Error deleting login history")
                > 0
        } else {
            false
        }
    }

    fn get_user_session(&self, user: Uuid) -> AppResult<Vec<LoginHistory>> {
        if let Ok(data) = login_history
            .filter(login_history_user_id.eq(user))
            .load::<LoginHistory>(&mut self.source.get().unwrap())
        {
            Ok(data)
        } else {
            Err(APIError::InternalError)
        }
    }

    fn login(&self, params: LoginParams) -> AppResult<AuthEntity> {
        if let Ok(user) = users::table
            .filter(email.eq(&params.email))
            .get_result::<User>(&mut self.source.get().unwrap())
        {
            if !user.password.is_empty() && verify(&params.password, &user.password).unwrap() {
                return if let Ok(login_session) = self.add_user_session(user.id, params) {
                    let login_info = LoginInfo {
                        id: user.id.to_string(),
                        email: user.email,
                        login_session: login_session.id,
                    };

                    match AuthToken::generate_token(&login_info) {
                        Ok(token) => Ok(AuthEntity::new(token)),
                        Err(e) => Err(e),
                    }
                } else {
                    Err(APIError::InternalError)
                };
            }
            return Err(APIError::InvalidCredentials);
        }

        Err(APIError::InvalidCredentials)
    }

    fn general_token(&self, params: GeneralTokenParams) -> AppResult<AuthEntity> {
        if params.verify() {
            match GeneralToken::generate_general_token() {
                Ok(token) => Ok(AuthEntity::new(token)),
                Err(e) => Err(e),
            }
        } else {
            Err(APIError::InvalidCredentials)
        }
    }

    fn is_valid_login_session(&self, user: Uuid, login_session: Uuid) -> bool {
        login_history::table
            .filter(login_history_user_id.eq(&user))
            .filter(login_history_id.eq(&login_session))
            .execute(&mut self.source.get().unwrap())
            .is_ok()
    }

    fn update_password(&self, user: Uuid, params: UpdatePasswordParams) -> AppResult<()> {
        if let Ok(user) = users::table
            .filter(user_id.eq(user))
            .get_result::<User>(&mut self.source.get().unwrap())
        {
            if !params.old_password.is_empty()
                && verify(&params.old_password, &user.password).unwrap()
            {
                let new_password = bcrypt::hash(&params.new_password, DEFAULT_COST).unwrap();
                diesel::update(users::table)
                    .filter(user_id.eq(&user.id))
                    .set(password.eq(&new_password))
                    .execute(&mut self.source.get().unwrap())
                    .expect("Error updating user password");
                return Ok(());
            }
            return Err(APIError::InvalidCredentials);
        } else {
            Err(APIError::InternalError)
        }
    }
}
