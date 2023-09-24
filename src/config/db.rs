use diesel::{pg::PgConnection, r2d2::{self, ConnectionManager}};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        Config {
            database_url,
            jwt_secret,
        }
    }
}

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