use diesel::{PgConnection, r2d2, r2d2::ConnectionManager};

use crate::core::error::APIError;

pub type AppResult<T> = Result<T, APIError>;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<PgConnection>;
pub type DBConn = PostgresPool;

