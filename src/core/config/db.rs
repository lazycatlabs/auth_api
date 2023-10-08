use std::env;

use diesel::{
    PgConnection,
    r2d2::ConnectionManager,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::core::types::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();


pub fn init_db() -> DBConn {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let pool = init_db_pool(&db_url);
    // Run the migration
    run_migration(&mut pool.get().unwrap());

    return pool;
}

pub fn init_db_pool(url: &str) -> DBConn {
    let con_manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .build(con_manager)
        .expect("Could not build connection pool.")
}

pub fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");
}