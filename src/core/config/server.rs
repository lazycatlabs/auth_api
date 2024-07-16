use std::env;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

use crate::core::{
    config::{db::init_db, routes::config_services},
    middlewares::cors::cors,
};

pub async fn run() -> std::io::Result<()> {
    let app_host = env::var("APP_HOST").unwrap_or(String::from("127.0.0.1"));
    let app_port = env::var("APP_PORT").unwrap_or(String::from("8080"));
    let app_url = format!("{}:{}", &app_host, &app_port);

    // injection
    let state = {
        let pool = init_db();
        use crate::core::middlewares::state::AppState;
        AppState::new(pool)
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(cors())
            .app_data(Data::new(state.clone()))
            .configure(config_services)
    })
    .bind(&app_url)?
    .run()
    .await
}
