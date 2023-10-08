use crate::{
    features::{
        auth::{
            data::repository::auth::AuthRepository,
            domain::usecase::service::AuthService,
        },
        user::{
            data::repository::user::UserRepository,
            domain::usecase::service::UserService,
        },
    },
};
use crate::core::types::DBConn;

#[derive(Clone)]
pub struct DiContainer {
    pub user_service: UserService<UserRepository>,
    pub auth_service: AuthService<AuthRepository>,
}

impl DiContainer {
    pub fn new(db_conn: &DBConn) -> Self {
        // user
        let user_repository = UserRepository::new(db_conn.clone());
        let user_service = UserService::new(user_repository.clone());

        // auth
        let auth_repository = AuthRepository::new(db_conn.clone());
        let auth_service = AuthService::new(auth_repository);

        Self {
            user_service,
            auth_service,
        }
    }
}
