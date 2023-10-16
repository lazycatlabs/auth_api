use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    pub id: String,
    pub email: String,
    pub login_session: Uuid,
}