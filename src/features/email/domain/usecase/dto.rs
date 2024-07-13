use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmailParams {
    #[validate(length(min = 0, message = "Can't be empty"))]
    pub email: Option<String>,
    #[validate(length(min = 0, message = "Can't be empty"))]
    pub subject: Option<String>,
    #[validate(length(min = 0, message = "Can't be empty"))]
    pub body: Option<String>,
}