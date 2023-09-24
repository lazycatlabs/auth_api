use std::env;

use actix_cors::Cors;
use actix_web::{App, http::header, HttpServer, middleware::Logger, web};
use dotenv::dotenv;

use crate::config::db::Config;

mod models;
mod schema;
mod config;
mod api;
mod constants;
mod error;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();
    Config::init();

    let app_host = env::var("APP_HOST").unwrap_or(String::from("127.0.0.1"));
    let app_port = env::var("APP_PORT").unwrap_or(String::from("8080"));
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let pool = config::db::init_db_pool(&db_url);
    // Run the migration
    config::db::run_migration(&mut pool.get().unwrap());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::default()
                      .send_wildcard()
                      .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                      .allowed_headers(vec![
                          header::AUTHORIZATION,
                          header::ACCEPT,
                          header::CONTENT_TYPE])
                      .allowed_header(header::CONTENT_TYPE)
                      .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(config::app::config_services)
    })
        .bind(&app_url)?
        .run()
        .await
}
