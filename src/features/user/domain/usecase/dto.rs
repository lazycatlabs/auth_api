use chrono::Utc;
use diesel::AsChangeset;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    features::user::data::models::user::User,
    schema::users,
};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterParams {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 20))]
    pub name: String,
    #[validate(length(min = 3, max = 20))]
    pub password: String,
}

impl From<RegisterParams> for User {
    fn from(params: RegisterParams) -> Self {
        User {
            id: Uuid::new_v4(),
            email: params.email,
            name: params.name,
            password: params.password,
            role: String::from("user"),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            login_session: String::from(""),
            photo: String::from("default.png"),
            verified: false,
        }
    }
}


#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct UpdateUserParams {
    pub name: Option<String>,
    pub photo: Option<String>,
    pub verified: Option<bool>,
}