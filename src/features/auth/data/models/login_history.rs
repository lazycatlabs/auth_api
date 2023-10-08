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
    pub id: i32,
    pub user_id: Uuid,
    pub login_timestamp: NaiveDateTime,
}
