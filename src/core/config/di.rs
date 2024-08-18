use std::sync::Arc;

use crate::{
    core::types::DBConn,
    features::{
        auth::{data::repository::auth_repository_impl::AuthRepository, domain::usecase::service::AuthService},
        user::{data::repository::user_repository_impl::UserRepository, domain::usecase::service::UserService},
    },
};

#[derive(Clone)]
pub struct DiContainer {
    pub user_service: UserService,
    pub auth_service: AuthService,
}

impl DiContainer {
    pub fn new(db_conn: &DBConn) -> Self {
        // user
        let user_repository = UserRepository::new(db_conn.clone());
        let user_service = UserService::new(Arc::new(user_repository.clone()));

        // auth
        let auth_repository = AuthRepository::new(db_conn.clone());
        let auth_service = AuthService::new(
            Arc::new(auth_repository.clone()),
            Arc::new(user_repository.clone()),
        );

        Self {
            user_service,
            auth_service,
        }
    }
}
