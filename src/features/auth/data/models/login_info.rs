use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    pub id: String,
    pub email: String,
    pub login_session: String,
}