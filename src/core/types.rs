use diesel::{PgConnection, r2d2, r2d2::ConnectionManager};

use crate::core::error::APIError;

pub type AppResult<T> = Result<T, APIError>;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;
pub type Connection = PgConnection;

