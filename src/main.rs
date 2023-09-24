use std::env;

use actix_web::{App, get, HttpResponse, HttpServer, Responder,middleware::Logger};
use dotenv::dotenv;
use env_logger::Logger;

use crate::config::db::Config;
use crate::models::response::{Diagnostic, ResponseBodyNoData};

mod models;
mod schema;
mod config;
mod errors;
mod api;

#[get("/api/healthchecker")]
async fn healthchecker() -> impl Responder {
    const MESSAGE: &str = "Hello, Lzyct";

    HttpResponse::Ok().json(ResponseBodyNoData::new(Diagnostic::new("200", MESSAGE)))
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();
    Config::init();


    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let pool = config::db::init_db_pool(&db_url);
    config::db::run_migration(&mut pool.get().unwrap());

    let server = HttpServer::new(move||{
        // App::new()
        //     .wrap(Logger::default())
        //     .wrap()
    });
}
