use std::env;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    App,
    http::header,
    HttpServer,
    middleware::Logger,
    web::Data,
};

use crate::core::config;
use crate::core::config::db::PostgresDatabase;
use crate::core::injection::Injection;
use crate::features::auth::domain::usecase::interface::IAuthService;
use crate::features::user::domain::usecase::interface::IUserService;

pub async fn run() -> std::io::Result<()> {
    let app_host = env::var("APP_HOST").unwrap_or(String::from("127.0.0.1"));
    let app_port = env::var("APP_PORT").unwrap_or(String::from("8080"));
    let app_url = format!("{}:{}", &app_host, &app_port);

    // injection
    let postgres = PostgresDatabase::new();
    let inject = Injection::new(&postgres);

    // auth
    let auth_service: Arc<dyn IAuthService> = Arc::new(inject.auth_service);
    let auth_service_data: Data<dyn IAuthService> = Data::from(auth_service);

    // user
    let user_service: Arc<dyn IUserService> = Arc::new(inject.user_service);
    let user_service_data: Data<dyn IUserService> = Data::from(user_service);


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
            .app_data(auth_service_data.clone())
            .app_data(user_service_data.clone())
            .configure(config::routes::config_services)
    })
        .bind(&app_url)?
        .run()
        .await
}