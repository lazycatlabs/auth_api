use validator::Validate;

use crate::{
    camel_case_struct,
    core::types::AppResult,
    features::auth::{
        data::repository::auth_repository_impl::AuthRepository,
        domain::{
            entity::auth_response::AuthResponse, repository::auth_repository::AuthRepositoryImpl,
        },
    },
};

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

pub fn general_token(
    auth_repository: &AuthRepository,
    token: GeneralTokenParams,
) -> AppResult<AuthResponse> {
    auth_repository.general_token(token)
}
