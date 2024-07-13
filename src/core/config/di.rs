use std::sync::Arc;

use crate::{
    core::types::DBConn,
    features::{
        auth::{
            data::repository::auth::AuthRepository,
            domain::usecase::service::AuthService,
        },
        email::{
            data::repository::email::EmailRepository,
            domain::usecase::service::EmailService,
        },
        user::{
            data::repository::user::UserRepository,
            domain::usecase::service::UserService,
        },
    },
};

#[derive(Clone)]
pub struct DiContainer {
    pub user_service: UserService,
    pub auth_service: AuthService,
    pub email_service: EmailService,
}

impl DiContainer {
    pub fn new(db_conn: &DBConn) -> Self {
        // user
        let user_repository = UserRepository::new(db_conn.clone());
        let user_service =
            UserService::new(Arc::new(user_repository.clone()));

        // auth
        let auth_repository = AuthRepository::new(db_conn.clone());
        let auth_service =
            AuthService::new(
                Arc::new(auth_repository.clone()),
                Arc::new(user_repository.clone()));

        // email
        let email_repository = EmailRepository::new(db_conn.clone());
        let email_service =
            EmailService::new(
                Arc::new(email_repository.clone()),
                Arc::new(user_repository.clone()));

        Self {
            user_service,
            auth_service,
            email_service,
        }
    }
}
