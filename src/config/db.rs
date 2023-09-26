use diesel::{pg::PgConnection, r2d2::{self, ConnectionManager}};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();


// database configuration
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;
pub type Connection = PgConnection;

pub fn init_db_pool(url: &str) -> Pool {
    let con_manager = ConnectionManager::<Connection>::new(url);
    Pool::builder()
        .build(con_manager)
        .expect("Could not build connection pool.")
}

pub fn run_migration(conn: &mut Connection) {
    conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");
}