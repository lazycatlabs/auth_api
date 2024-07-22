use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    core::{error::APIError, types::AppResult},
    schema::users,
};

#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = users)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub photo: String,
    pub verified: bool,
    #[serde(skip_serializing)]
    pub password: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn hash_password(&mut self) -> AppResult<()> {
        if let Ok(hashed_password) = hash(self.password.as_bytes(), DEFAULT_COST) {
            self.password = hashed_password;
            Ok(())
        } else {
            Err(APIError::InternalError)
        }
    }
}
