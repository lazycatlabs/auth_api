use async_trait::async_trait;
use chrono::Utc;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    core::{
        constants::MESSAGE_SUCCESS,
        error::APIError,
        types::{
            AppResult,
            DBConn,
        },
    },
    features::user::{
        data::models::user::User,
        domain::{
            entity::user::UserEntity,
            repository::user::IUserRepository,
            usecase::dto::{
                RegisterParams,
                UpdateUserParams,
            },
        },
    },
    schema::users::{self, dsl::*},
};

#[derive(Clone)]
pub struct UserRepository {
    source: DBConn,
}

impl UserRepository {
    pub fn new(source: DBConn) -> Self {
        UserRepository { source }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    fn create(&self, params: RegisterParams) -> AppResult<String> {
        let mut user = User::from(params);
        let _ = user.hash_password();

        println!("create ");

        match diesel::insert_into(users::table)
            .values(&user)
            .execute(&mut self.source.get().unwrap())
        {
            Ok(_) => Ok(MESSAGE_SUCCESS.to_string()),
            Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                )) => Err(APIError::BadRequest { message: format!("Email '{}' already exists.", user.email) }),
            Err(_) => Err(APIError::BadRequest { message: "MESSAGE_INTERNAL_ERROR".to_string() }),
        }
    }

    fn find_user_by_id(&self, user_id: Uuid) -> AppResult<UserEntity> {
        match users::table
            .filter(id.eq(user_id))
            .get_result::<User>(&mut self.source.get().unwrap()) {
            Ok(user) => Ok(
                UserEntity {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                    photo: user.photo,
                    verified: user.verified,
                }
            ),
            Err(_) => Err(APIError::UserNotFoundError),
        }
    }

    fn update_user(&self, user_id: Uuid, params: UpdateUserParams) -> AppResult<UserEntity> {
        if let Ok(user) = self.find_user_by_id(user_id) {
            match diesel::update(users.find(user.id))
                .set((
                    name.eq(params.name.unwrap_or(user.name)),
                    photo.eq(params.photo.unwrap_or(user.photo)),
                    verified.eq(params.verified.unwrap_or(user.verified)),
                    updated_at.eq(Utc::now().naive_utc()),
                ))
                .get_result::<User>(&mut self.source.get().unwrap()) {
                Ok(updated_user) => Ok(UserEntity {
                    id: updated_user.id,
                    name: updated_user.name,
                    email: updated_user.email,
                    photo: updated_user.photo,
                    verified: updated_user.verified,
                }),
                Err(_) => Err(APIError::InternalError)
            }
        } else {
            Err(APIError::UserNotFoundError)
        }
    }

    fn delete(&self, user_id: Uuid) -> AppResult<String> {
        if let Ok(user) = self.find_user_by_id(user_id) {
            match diesel::delete(users.find(user.id))
                .execute(&mut self.source.get().unwrap()) {
                Ok(_) => Ok(format!("User with email '{}' deleted successfully", user.email)),
                Err(err) => {
                    println!("err: {:?}", err);
                    Err(APIError::InternalError)
                }
            }
        } else {
            Err(APIError::UserNotFoundError)
        }
    }
}