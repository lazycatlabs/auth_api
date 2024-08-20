use crate::{
    core::types::DBConn,
    features::{
        auth::data::repository::auth_repository_impl::AuthRepository,
        user::data::repository::user_repository_impl::UserRepository,
    },
};

#[derive(Clone)]
pub struct DiContainer {
    pub auth_repository: AuthRepository,
    pub user_repository: UserRepository,
}

impl DiContainer {
    pub fn new(db_conn: &DBConn) -> Self {
        // user
        let user_repository = UserRepository::new(db_conn.clone());

        // auth
        let auth_repository = AuthRepository::new(db_conn.clone());

        Self {
            auth_repository,
            user_repository,
        }
    }
}
