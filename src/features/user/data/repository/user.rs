use async_trait::async_trait;
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
            repository::user::IUserRepository,
            usecase::dto::RegisterParams,
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
    async fn create(&self, params: RegisterParams) -> AppResult<String> {
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

    async fn find_user_by_id(&self, user_id: Uuid) -> AppResult<User> {
        match users::table
            .filter(id.eq(user_id))
            .get_result::<User>(&mut self.source.get().unwrap()) {
            Ok(user) => Ok(user),
            Err(_) => Err(APIError::UserNotFoundError),
        }
    }
}