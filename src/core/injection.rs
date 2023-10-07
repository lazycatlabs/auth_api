use crate::core::config::db::PostgresDatabase;
use crate::features::{
    auth::{
        data::repository::auth::AuthRepository,
        domain::usecase::service::AuthService,
    },
    user::{
        data::repository::user::UserRepository,
        domain::usecase::service::UserService,
    },
};

pub struct Injection<'user, 'auth> {
    pub user_service: UserService<'user, UserRepository<'user>>,
    pub auth_service: AuthService<AuthRepository<'auth, 'user>>,
}

impl<'user, 'auth> Injection<'user, 'auth> {
    pub fn new(db: &'user PostgresDatabase) -> Self {
        // user
        let user_repository = UserRepository::new(&db);
        let user_service = UserService::new(&user_repository);

        // auth
        let auth_repository = AuthRepository::new(&db, &user_repository);
        let auth_service = AuthService::new(auth_repository);

        Self {
            user_service,
            auth_service,
        }
    }
}
