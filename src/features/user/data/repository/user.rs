use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};
use uuid::Uuid;

use crate::{
    core::{
        constants::MESSAGE_SUCCESS,
        error::APIError,
        types::{AppResult, DBConn},
    },
    features::user::{
        data::models::user::User,
        domain::{
            entity::user::UserEntity,
            repository::user::IUserRepository,
            usecase::dto::{RegisterParams, UpdateUserParams},
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

impl IUserRepository for UserRepository {
    fn create(&self, params: RegisterParams) -> AppResult<String> {
        let mut user = User::from(params);
        let _ = user.hash_password();

        diesel::insert_into(users::table)
            .values(&user)
            .execute(&mut self.source.get().unwrap())
            .map(|_| MESSAGE_SUCCESS.to_string())
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => APIError::BadRequest {
                    message: format!("Email '{}' already exists.", user.email),
                },
                _ => APIError::InternalError,
            })
    }

    fn find_user_by_id(&self, user_id: Uuid) -> AppResult<UserEntity> {
        users::table
            .filter(id.eq(user_id))
            .get_result::<User>(&mut self.source.get().unwrap())
            .map(|user| UserEntity {
                id: user.id,
                name: user.name,
                email: user.email,
                photo: user.photo,
                verified: user.verified,
            })
            .map_err(|_| APIError::UserNotFoundError)
    }

    fn update_user(&self, user_id: Uuid, params: UpdateUserParams) -> AppResult<UserEntity> {
        self.find_user_by_id(user_id)
            .map(|user| {
                diesel::update(users.find(user.id))
                    .set((
                        name.eq(params.name.unwrap_or(user.name)),
                        photo.eq(params.photo.unwrap_or(user.photo)),
                        verified.eq(params.verified.unwrap_or(user.verified)),
                        updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .get_result::<User>(&mut self.source.get().unwrap())
                    .map(|updated_user| UserEntity {
                        id: updated_user.id,
                        name: updated_user.name,
                        email: updated_user.email,
                        photo: updated_user.photo,
                        verified: updated_user.verified,
                    })
                    .map_err(|_| APIError::InternalError)
            })
            .map_err(|_| APIError::UserNotFoundError)?
    }

    fn delete(&self, user_id: Uuid) -> AppResult<String> {
        self.find_user_by_id(user_id)
            .map(|user| {
                diesel::delete(users.find(user.id))
                    .execute(&mut self.source.get().unwrap())
                    .map(|_| format!("User with email '{}' deleted successfully", user.email))
                    .map_err(|_| APIError::InternalError)
            })
            .map_err(|_| APIError::UserNotFoundError)?
    }
}
