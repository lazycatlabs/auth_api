use chrono::Utc;
use diesel::AsChangeset;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{camel_case_struct, features::user::data::models::user::User, schema::users};

camel_case_struct!(RegisterParams {
    #[validate(
      required(message = "field is required"),
      email(message = "Invalid email"),
    )]
    email: Option<String>,
    #[validate(
      required(message = "field is required"),
      length(min = 1, message = "Can't be empty"),
    )]
    name: Option<String>,
    #[validate(
      required(message = "field is required"),
      length(min = 3,max = 20,message = "Password must be between 3 and 20 characters"),
    )]
    password: Option<String>
});

impl From<RegisterParams> for User {
    fn from(params: RegisterParams) -> Self {
        User {
            id: Uuid::new_v4(),
            email: params.email.unwrap(),
            name: params.name.unwrap(),
            password: params.password.unwrap(),
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
