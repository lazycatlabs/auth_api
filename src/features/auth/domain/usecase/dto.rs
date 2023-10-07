use chrono::NaiveDateTime;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::schema::login_history;

#[derive(Insertable)]
#[table_name = "login_history"]
pub struct LoginHistoryParams {
    pub user_id: Uuid,
    pub login_timestamp: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct LoginParams {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 3, max = 20))]
    pub password: String,
}