use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    camel_case_struct,
    core::{error::APIError, types::AppResult},
    features::user::{
        data::{models::user::User, repository::user_repository_impl::UserRepository},
        domain::{
            entity::user_response::UserResponse, repository::user_repository::UserRepositoryImpl,
        },
    },
};

camel_case_struct!(RegisterParams {
  #[validate(
    required(message = "field is required"),
    email(message = "Invalid email"),
  )]
  email: Option<String>,
  #[validate(
    required(message = "field is required"),
    length(min = 1, message = "Can't be empty"),
  )]
  name: Option<String>,
  #[validate(
    required(message = "field is required"),
    length(min = 3,max = 20,message = "Password must be between 3 and 20 characters"),
  )]
  password: Option<String>,
  photo: Option<String>,
});

impl From<RegisterParams> for User {
    fn from(params: RegisterParams) -> Self {
        let default_photo = String::from("https://user-images.githubusercontent.com/1531684/281937715-f53c55be-4b70-43b5-bb50-11706fb71ada.png");
        User {
            id: Uuid::new_v4(),
            email: params.email.unwrap(),
            name: params.name.unwrap(),
            password: params.password.unwrap(),
            role: String::from("user"),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            photo: params.photo.unwrap_or(default_photo),
            verified: false,
        }
    }
}

pub fn register(
    user_repository: &UserRepository,
    params: RegisterParams,
) -> AppResult<UserResponse> {
    params
        .validate()
        .map(|_| user_repository.create(params))
        .map_err(|e| APIError::BadRequest {
            message: e.to_string(),
        })?
}
