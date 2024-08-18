use chrono::NaiveDateTime;
use diesel::Insertable;
use uuid::Uuid;
use validator::Validate;

use crate::{camel_case_struct, schema::login_history};

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



camel_case_struct!(GeneralTokenParams {
  #[validate(
    required(message = "field is required"),
    length(min = 1, message = "Can't be empty"),
  )]
   client_id: Option<String>,
  #[validate(
    required(message = "field is required"),
    length(min = 1, message = "Can't be empty"),
  )]
   client_secret: Option<String>
});

impl GeneralTokenParams {
    pub fn verify(&self) -> bool {
        self.validate().is_ok()
            && self.client_id.as_deref() == Some(env!("CLIENT_ID"))
            && self.client_secret.as_deref() == Some(env!("CLIENT_SECRET"))
    }
}

camel_case_struct!(UpdatePasswordParams {
    #[validate(
      required(message = "field is required"),
      length(min = 1, message = "Can't be empty"),
    )]
    old_password: Option<String>,
    #[validate(
      required(message = "field is required"),
      length(min = 6, message = "Must be at least 6 characters"),
      must_match(other = "confirm_password", message = "Password not match")
    )]
    new_password: Option<String>,
    #[validate(
      required(message = "field is required"),
      length(min = 6, message = "Must be at least 6 characters"),
      must_match(other = "new_password", message = "Password not match")
    )]
    confirm_password: Option<String>
});
