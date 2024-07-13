use dotenv_codegen::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthEntity {
    pub token: String,
    pub token_type: String,
}

impl AuthEntity {
    pub fn new(token: String) -> Self {
        Self {
            token: format!("{}.{}", dotenv!("TOKEN_PREFIX"), token),
            token_type: "Bearer".to_string(),
        }
    }
}