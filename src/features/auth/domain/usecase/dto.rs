use chrono::NaiveDateTime;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::schema::login_history;

#[derive(Insertable)]
#[diesel(table_name = login_history)]
pub struct LoginHistoryParams {
    pub user_id: Uuid,
    pub login_timestamp: NaiveDateTime,
    pub ip_addr: String,
    pub device_info: String,
    pub os_info: String,
    pub fcm_token: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginParams {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 3, max = 20))]
    pub password: String,
    #[validate(length(min = 0, message = "Can't be empty"))]
    pub ip_addr: Option<String>,
    #[validate(length(min = 0, message = "Can't be empty"))]
    pub device_info: String,
    #[validate(length(min = 0, message = "Can't be empty"))]
    pub os_info: String,
    #[validate(length(min = 0, message = "Can't be empty"))]
    pub fcm_token: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeneralTokenParams {
    #[validate(length(min = 0, message = "Can't be empty"))]
    pub client_id: Option<String>,
    #[validate(length(min = 0, message = "Can't be empty"))]
    pub client_secret: Option<String>,
}

impl GeneralTokenParams {
    pub fn verify(&self) -> bool {
        self.validate().is_ok()
            && self.client_id.as_deref() == Some(env!("CLIENT_ID"))
            && self.client_secret.as_deref() == Some(env!("CLIENT_SECRET"))
    }
}


#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePasswordParams {
    #[validate(length(min = 1, message = "Can't be empty"))]
    pub old_password: String,
    #[validate(length(min = 6, message = "Must be at least 6 characters"),
    must_match(other = "confirm_password", message = "Password not match"))]
    pub new_password: String,
    #[validate(length(min = 6, message = "Must be at least 6 characters"),
    must_match(other = "new_password", message = "Password not match"))]
    pub confirm_password: String,
}