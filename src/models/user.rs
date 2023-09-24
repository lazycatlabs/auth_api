use argon2::Config;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rand::random;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::db::Connection,
    errors::user::ServiceError,
    schema::users::{self},
};

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: Uuid,
    name: String,
    email: String,
    photo: String,
    verified: bool,
    #[serde(skip_serializing)]
    password: String,
    role: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
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

impl User {
    pub fn signup(new_user: UserDTO, conn: &mut Connection) -> Result<Self, ServiceError> {
        let mut user = User::from(new_user);
        let _ = user.hash_password();

        match diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)
        {
            Ok(user) => Ok(user),
            Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                )) => Err(ServiceError::EmailAlreadyExistsError { email: user.email }),
            Err(_) => Err(ServiceError::InternalError)
        }
    }

    pub fn hash_password(&mut self) -> Result<(), ServiceError> {
        let salt: [u8; 32] = random();
        let cfg = Config::default();

        if let Ok(hashed_password) = argon2::hash_encoded(self.password.as_bytes(), &salt, &cfg) {
            self.password = hashed_password;
            Ok(())
        } else {
            Err(ServiceError::InternalError)
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
            photo: String::from("default.png"),
            verified: false,
        }
    }
}