use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub photo: String,
    pub verified: bool,
}