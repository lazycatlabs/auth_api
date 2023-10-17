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
    pub fcm_token:String,
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