use chrono::Utc;
use diesel::AsChangeset;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{camel_case_struct, features::user::data::models::user::User, schema::users};

camel_case_struct!(RegisterParams {
    #[validate(email(message = "Invalid email"))]
    email: String,
    #[validate(length(min = 0, message = "Can't be empty"))]
    name: String,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Password must be between 3 and 20 characters"
    ))]
    password: String
});

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
            photo: String::from("default.png"),
            verified: false,
        }
    }
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct UpdateUserParams {
    pub name: Option<String>,
    pub photo: Option<String>,
    pub verified: Option<bool>,
}
