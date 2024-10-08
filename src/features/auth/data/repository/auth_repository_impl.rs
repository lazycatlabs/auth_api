use bcrypt::{verify, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};
use jsonwebtoken::TokenData;
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
                entity::auth_response::AuthResponse,
                repository::auth_repository::AuthRepositoryImpl,
                usecases::dto::*,
                usecases::{
                    general_token::GeneralTokenParams, login::LoginParams,
                    update_password::UpdatePasswordParams,
                },
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

impl AuthRepositoryImpl for AuthRepository {
    fn add_user_session(&self, user: Uuid, login_params: LoginParams) -> AppResult<LoginHistory> {
        // get user information by id
        let now = Utc::now().naive_utc();
        let login_history_params = LoginHistoryParams {
            user_id: user,
            ip_addr: login_params.ip_addr.unwrap(),
            os_info: login_params.os_info.unwrap(),
            device_info: login_params.device_info.unwrap(),
            login_timestamp: now,
            fcm_token: login_params.fcm_token.unwrap(),
        };

        diesel::insert_into(login_history::table)
            .values(&login_history_params)
            .get_result::<LoginHistory>(&mut self.source.get().unwrap())
            .map_err(|_| APIError::InternalError)
    }

    fn remove_user_session(&self, user: Uuid, login_session: Uuid) -> bool {
        self.is_valid_login_session(user, login_session)
            .then(|| {
                diesel::delete(
                    login_history::table
                        .filter(login_history_user_id.eq(user))
                        .filter(login_history_id.eq(login_session)),
                )
                .execute(&mut self.source.get().unwrap())
                .map(|effected_row| effected_row > 0)
                .unwrap_or(false)
            })
            .unwrap_or(false)
    }

    fn get_user_session(&self, user: Uuid) -> AppResult<Vec<LoginHistory>> {
        login_history
            .filter(login_history_user_id.eq(user))
            .load::<LoginHistory>(&mut self.source.get().unwrap())
            .map_err(|_| APIError::InternalError)
    }

    fn login(&self, params: LoginParams) -> AppResult<AuthResponse> {
        let param = params.clone();
        let email_param = param.email.as_deref().unwrap_or("");
        let password_param = param.password.as_deref().unwrap_or("");

        users::table
            .filter(email.eq(&email_param))
            .get_result::<User>(&mut self.source.get().unwrap())
            .map(|user| {
                (!user.password.is_empty() && verify(password_param, &user.password).unwrap())
                    .then(|| {
                        self.add_user_session(user.id, params)
                            .map(|login_session| {
                                let login_info = LoginInfo {
                                    id: user.id.to_string(),
                                    email: user.email,
                                    login_session: login_session.id,
                                };
                                AuthToken::generate_token(&login_info).map(AuthResponse::new)
                            })
                            .unwrap_or(Err(APIError::InternalError))
                    })
                    .unwrap_or(Err(APIError::InvalidCredentials))
            })
            .map_err(|_| APIError::UserNotFoundError)?
    }

    fn general_token(&self, params: GeneralTokenParams) -> AppResult<AuthResponse> {
        params
            .verify()
            .then(|| GeneralToken::generate_general_token().map(AuthResponse::new))
            .unwrap_or(Err(APIError::InvalidCredentials))
    }

    fn is_valid_login_session(&self, user: Uuid, login_session: Uuid) -> bool {
        !login_history::table
            .filter(login_history_user_id.eq(&user))
            .filter(login_history_id.eq(&login_session))
            .load::<LoginHistory>(&mut self.source.get().unwrap())
            .map_err(|_| APIError::InternalError)
            .unwrap()
            .is_empty()
    }

    fn update_password(&self, user: Uuid, params: UpdatePasswordParams) -> AppResult<()> {
        let user = users::table
            .filter(user_id.eq(user))
            .get_result::<User>(&mut self.source.get().unwrap())
            .map_err(|_| APIError::UserNotFoundError)?;

        let current_password = &params.old_password.unwrap_or("".to_string());
        let new_password_param = &params.new_password.unwrap_or("".to_string());

        // validate current password
        if !verify(current_password, &user.password).unwrap() {
            return Err(APIError::BadRequest {
                message: "Invalid current password".to_string(),
            });
        }

        let new_password = bcrypt::hash(new_password_param, DEFAULT_COST).unwrap();
        diesel::update(users::table)
            .filter(user_id.eq(&user.id))
            .set(password.eq(&new_password))
            .execute(&mut self.source.get().unwrap())
            .expect("Error updating user password");
        Ok(())
    }

    fn verify_token(&self, params: &TokenData<AuthToken>) -> AppResult<Uuid> {
        self.is_valid_login_session(params.claims.jti, params.claims.login_session)
            .then_some(params.claims.jti)
            .ok_or(APIError::Unauthorized)
    }
}
