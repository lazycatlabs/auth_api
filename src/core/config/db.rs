use std::env;

use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::core::types::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();


// database configuration
#[derive(Clone)]
pub struct PostgresDatabase<> {
    pub pool: Pool,
}

impl PostgresDatabase {
    pub fn new() -> Self {
        let pool = init_db();
        Self {
            pool
        }
    }
}

pub fn init_db() -> Pool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let pool = init_db_pool(&db_url);
    // Run the migration
    run_migration(&mut pool.get().unwrap());

    return pool;
}

pub fn init_db_pool(url: &str) -> Pool {
    let con_manager = ConnectionManager::<Connection>::new(url);
    Pool::builder()
        .build(con_manager)
        .expect("Could not build connection pool.")
}

pub fn run_migration(conn: &mut Connection) {
    conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");
}