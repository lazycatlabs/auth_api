use uuid::Uuid;
use validator::Validate;

use crate::{
    camel_case_struct,
    core::{error::APIError, types::AppResult},
    features::auth::{
        data::repository::auth_repository_impl::AuthRepository,
        domain::repository::auth_repository::AuthRepositoryImpl,
    },
};

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

pub fn update_password(
    auth_repository: &AuthRepository,
    user_id: Uuid,
    params: UpdatePasswordParams,
) -> AppResult<()> {
    params
        .validate()
        .map_err(|e| APIError::BadRequest {
            message: e.to_string(),
        })
        .and_then(|_| {
            (params.old_password != params.new_password)
                .then(|| auth_repository.update_password(user_id, params))
                .ok_or(APIError::BadRequest {
                    message: "Old password and new password must be different".to_string(),
                })
        })?
}
