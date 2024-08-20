use chrono::NaiveDateTime;
use diesel::Insertable;
use uuid::Uuid;

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
