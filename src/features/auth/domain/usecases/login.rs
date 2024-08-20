use validator::Validate;

use crate::{
    camel_case_struct,
    core::{error::APIError, types::AppResult},
    features::auth::{
        data::repository::auth_repository_impl::AuthRepository,
        domain::{
            entity::auth_response::AuthResponse, repository::auth_repository::AuthRepositoryImpl,
        },
    },
};

camel_case_struct!(LoginParams {
  #[validate(
    required(message = "field is required"),
    email(message = "Invalid email"),
  )]
   email: Option<String>,
  #[validate(
    required(message = "field is required"),
    length(min = 3, max = 20),
  )]
   password: Option<String>,
  #[validate(
    length(min = 1, message = "Can't be empty"),
  )]
   ip_addr: Option<String>,
  #[validate(
    required(message = "field is required"),
    length(min = 1, message = "Can't be empty"),
  )]
   device_info: Option<String>,
  #[validate(
    required(message = "field is required"),
    length(min = 1, message = "Can't be empty"),
  )]
   os_info: Option<String>,
  #[validate(
    required(message = "field is required"),
    length(min = 1, message = "Can't be empty"),
  )]
   fcm_token: Option<String>
});

pub fn login(auth_repository: &AuthRepository, params: LoginParams) -> AppResult<AuthResponse> {
    params
        .validate()
        .map_err(|e| APIError::BadRequest {
            message: e.to_string(),
        })
        .and_then(|_| auth_repository.login(params))
}
