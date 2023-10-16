use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Queryable};
use uuid::Uuid;

use crate::{
    features::user::data::models::user::User,
    schema::login_history,
};

#[derive(Identifiable, Associations, Queryable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = login_history)]
pub struct LoginHistory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub login_timestamp: NaiveDateTime,
    pub ip_address: String,
    pub device_info: String,
    pub os_info: String,
}
