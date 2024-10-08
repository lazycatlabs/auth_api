use std::env;

use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};
use uuid::Uuid;

use crate::features::user::domain::usecase::update_user::UpdateUserParams;
use crate::{
    core::{
        error::APIError,
        types::{AppResult, DBConn},
    },
    features::user::{
        data::models::user::User,
        domain::{
            entity::user_response::UserResponse,
            entity::user_response::UsersResponse,
            repository::user_repository::UserRepositoryImpl,
            usecase::{list_user::PaginationParams, register::RegisterParams},
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

impl UserRepositoryImpl for UserRepository {
    fn create(&self, params: RegisterParams) -> AppResult<UserResponse> {
        let mut user = User::from(params);
        let _ = user.hash_password();
        let email_register = user.email.clone();

        let is_allow_modified =
            env::var("IS_SUPPORT_MODIFIED").expect("DATABASE_URL not found.") == "true";

        if !is_allow_modified {
            // return mock user
            return Ok(UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                photo: user.photo,
                verified: user.verified,
                created_at: user.created_at,
                updated_at: user.updated_at,
            });
        }

        diesel::insert_into(users::table)
            .values(&user)
            .execute(&mut self.source.get().unwrap())
            .map(|_| UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                photo: user.photo,
                verified: user.verified,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => APIError::BadRequest {
                    message: format!("Email '{}' already exists.", email_register),
                },
                _ => APIError::InternalError,
            })
    }

    fn find_user_by_id(&self, user_id: Uuid) -> AppResult<UserResponse> {
        users::table
            .filter(id.eq(user_id))
            .get_result::<User>(&mut self.source.get().unwrap())
            .map(|user| UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                photo: user.photo,
                verified: user.verified,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
            .map_err(|_| APIError::UserNotFoundError)
    }

    fn update_user(&self, user_id: Uuid, params: UpdateUserParams) -> AppResult<UserResponse> {
        let is_allow_modified =
            env::var("IS_SUPPORT_MODIFIED").expect("DATABASE_URL not found.") == "true";

        self.find_user_by_id(user_id)
            .map(|user| {
                if !is_allow_modified {
                    // return mock response
                    return Ok(UserResponse {
                        id: user_id,
                        name: params.name.unwrap_or("".to_string()),
                        email: user.email.to_string(),
                        photo: params.photo.unwrap_or("".to_string()),
                        verified: params.verified.unwrap_or(false),
                        created_at: user.created_at,
                        updated_at: Utc::now().naive_utc(),
                    });
                }

                diesel::update(users.find(user.id))
                    .set((
                        name.eq(params.name.unwrap_or(user.name)),
                        photo.eq(params.photo.unwrap_or(user.photo)),
                        verified.eq(params.verified.unwrap_or(user.verified)),
                        updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .get_result::<User>(&mut self.source.get().unwrap())
                    .map(|updated_user| UserResponse {
                        id: updated_user.id,
                        name: updated_user.name,
                        email: updated_user.email,
                        photo: updated_user.photo,
                        verified: updated_user.verified,
                        created_at: updated_user.created_at,
                        updated_at: updated_user.updated_at,
                    })
                    .map_err(|_| APIError::InternalError)
            })
            .map_err(|_| APIError::UserNotFoundError)?
    }

    fn delete(&self, user_id: Uuid) -> AppResult<String> {
        let is_allow_modified =
            env::var("IS_SUPPORT_MODIFIED").expect("DATABASE_URL not found.") == "true";

        self.find_user_by_id(user_id)
            .map(|user| {
                if !is_allow_modified {
                    return Ok(format!(
                        "User with id '{}' deleted successfully",
                        user.email
                    ));
                }
                diesel::delete(users.find(user.id))
                    .execute(&mut self.source.get().unwrap())
                    .map(|_| format!("User with email '{}' deleted successfully", user.email))
                    .map_err(|_| APIError::InternalError)
            })
            .map_err(|_| APIError::UserNotFoundError)?
    }

    fn users(&self, params: PaginationParams) -> AppResult<UsersResponse> {
        // Calculate the offset (skip `page * per_page` results)
        let page = params.page.unwrap_or(1);
        let per_page = params.per_page.unwrap_or(20);
        let offset = (page - 1) * per_page;

        let total = users::table
            .count()
            .get_result::<i64>(&mut self.source.get().unwrap())
            .unwrap();

        // Fetch users with a limit and offset (pagination)
        users
            .limit(per_page)
            .offset(offset)
            .load::<User>(&mut self.source.get().unwrap())
            .map(|list_user| UsersResponse {
                users: list_user
                    .iter()
                    .map(|user| UserResponse {
                        id: user.id,
                        name: user.name.clone(),
                        email: user.email.clone(),
                        photo: user.photo.clone(),
                        verified: user.verified,
                        created_at: user.created_at,
                        updated_at: user.updated_at,
                    })
                    .collect(),
                total,
                page,
                per_page,
            })
            .map_err(|_| APIError::InternalError)
    }
}
