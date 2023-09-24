use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::errors::user::ServiceError;
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: Uuid,
    name: String,
    email: String,
    photo: String,
    verified: bool,
    password: String,
    role: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 20))]
    pub name: String,
    #[validate(length(min = 3, max = 20))]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Self>, ServiceError> {
        match users::table.load::<User>(conn) {
            Ok(users) => Ok(users),
            Err(_) => Err(ServiceError::InternalError),
        }
    }
}