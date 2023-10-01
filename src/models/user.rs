use std::future::Future;
use std::pin::Pin;

use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::Data;
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::db::{
        Connection,
        Pool,
    },
    constants::*,
    error::ServiceError, models::{
        jwt::UserToken,
        login_history::LoginHistory,
    },
    schema::users::{self, dsl::*},
    utils::token_utils::*,
};

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    email: String,
    name: String,
    photo: String,
    verified: bool,
    #[serde(skip_serializing)]
    password: String,
    role: String,
    login_session: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}


#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct UpdateUserDTO {
    pub name: Option<String>,
    pub photo: Option<String>,
    pub verified: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserDTO {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 20))]
    pub name: String,
    #[validate(length(min = 3, max = 20))]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginInfoDTO {
    pub id: String,
    pub email: String,
    pub login_session: String,
}

impl User {
    pub fn signup(new_user: UserDTO, conn: &mut Connection) -> Result<String, String> {
        let mut user = User::from(new_user);
        let _ = user.hash_password();

        match diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
        {
            Ok(_) => Ok(MESSAGE_SUCCESS.to_string()),
            Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                )) => Err(format!("Email '{}' already exists.", user.email)),
            Err(_) => Err("MESSAGE_INTERNAL_ERROR".to_string()),
        }
    }

    pub fn login(login: LoginDTO, conn: &mut Connection) -> Result<LoginInfoDTO, ServiceError> {
        if let Ok(user_verify) = users::table
            .filter(email.eq(&login.email))
            .get_result::<User>(conn)
        {
            if !user_verify.password.is_empty()
                && verify(&login.password, &user_verify.password).unwrap() {
                if let Ok(login_history) = LoginHistory::create(&user_verify.id, conn) {
                    if LoginHistory::save_login_history(login_history, conn).is_err() {
                        return Err(ServiceError::InternalError);
                    }
                    let login_session_str = User::generate_login_session();
                    if User::update_login_session_to_db(
                        &user_verify.id,
                        &login_session_str,
                        conn,
                    ) {
                        return Ok(LoginInfoDTO {
                            id: user_verify.id.to_string(),
                            email: user_verify.email,
                            login_session: login_session_str,
                        });
                    }
                }
            }
            return Err(ServiceError::InvalidCredentials);
        }

        Err(ServiceError::InvalidCredentials)
    }

    pub fn logout(user_id: Uuid, conn: &mut Connection) {
        if let Ok(user) = users::table.find(user_id).get_result::<User>(conn) {
            Self::update_login_session_to_db(&user.id, "", conn);
        }
    }

    pub fn find_user_by_id(user_id: &Uuid, conn: &mut Connection) -> Result<Self, ServiceError> {
        match users::table
            .filter(id.eq(user_id))
            .get_result::<User>(conn)
        {
            Ok(user) => Ok(user),
            Err(_) => Err(ServiceError::UserNotFoundError),
        }
    }

    fn update_login_session_to_db(
        user_id: &Uuid,
        login_session_str: &str,
        conn: &mut Connection,
    ) -> bool {
        if let Ok(user) = User::find_user_by_id(user_id, conn) {
            diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str.to_string()))
                .execute(conn)
                .is_ok()
        } else {
            false
        }
    }
    fn generate_login_session() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn is_valid_login_session(user_token: &UserToken, conn: &mut Connection) -> bool {
        users
            .filter(id.eq(&user_token.jti))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
    }

    fn hash_password(&mut self) -> Result<(), ServiceError> {
        if let Ok(hashed_password) = hash(&self.password.as_bytes(), DEFAULT_COST) {
            self.password = hashed_password;
            Ok(())
        } else {
            Err(ServiceError::InternalError)
        }
    }

    pub fn update_user(user_id: Uuid, user_update: UpdateUserDTO, conn: &mut Connection) -> Result<Self, ServiceError> {
        if let Ok(user) = User::find_user_by_id(&user_id, conn) {
            match diesel::update(users.find(user.id))
                .set((
                    name.eq(user_update.name.unwrap_or(user.name)),
                    photo.eq(user_update.photo.unwrap_or(user.photo)),
                    verified.eq(user_update.verified.unwrap_or(user.verified)),
                    updated_at.eq(Utc::now().naive_utc()),
                ))
                .get_result::<User>(conn) {
                Ok(updated_user) => Ok(updated_user),
                Err(_) => Err(ServiceError::InternalError)
            }
        } else {
            Err(ServiceError::UserNotFoundError)
        }
    }
}

// inject ::from with trait
impl From<UserDTO> for User {
    fn from(user: UserDTO) -> Self {
        User {
            id: Uuid::new_v4(),
            email: user.email,
            name: user.name,
            password: user.password,
            role: String::from("user"),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            login_session: String::from(""),
            photo: String::from("default.png"),
            verified: false,
        }
    }
}


///
impl FromRequest for User {
    type Error = ServiceError;
    type Future = Pin<Box<dyn Future<Output=Result<User, Self::Error>>>>;


    // act as auth middleware
    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(header_auth_string) = request.headers().get(AUTHORIZATION) {
            if let Some(pool) = request.app_data::<Data<Pool>>() {
                if let Ok(auth_str) = header_auth_string.to_str() {
                    if is_auth_header_valid(header_auth_string) {
                        let token = token_extractor(&auth_str);
                        if let Ok(token_data) = decode_token(&token.to_string()) {
                            if let Ok(user_id) = verify_token(&token_data, pool) {
                                if let Ok(user) = User::find_user_by_id(&user_id, &mut pool.get().unwrap()) {
                                    return Box::pin(async move {
                                        Ok(user)
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        Box::pin(async move { Err(ServiceError::Unauthorized) })
    }
}

