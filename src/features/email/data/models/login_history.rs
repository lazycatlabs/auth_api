use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    features::user::data::models::user::User,
    schema::login_history,
};

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = login_history)]
#[serde(rename_all = "camelCase")]
pub struct LoginHistory {
    pub id: Uuid,
    #[serde(skip_serializing)]
    pub user_id: Uuid,
    pub login_timestamp: NaiveDateTime,
    pub ip_address: String,
    pub device_info: String,
    pub os_info: String,
    pub fcm_token: String,
}
