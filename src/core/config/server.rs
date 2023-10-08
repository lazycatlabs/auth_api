use std::env;

use actix_cors::Cors;
use actix_web::{
    App,
    http::header,
    HttpServer,
    middleware::Logger,
};

use crate::core::config::{
    db::init_db,
    routes::config_services,
};

pub async fn run() -> std::io::Result<()> {
    let app_host = env::var("APP_HOST").unwrap_or(String::from("127.0.0.1"));
    let app_port = env::var("APP_PORT").unwrap_or(String::from("8080"));
    let app_url = format!("{}:{}", &app_host, &app_port);

    // injection
    let state = {
        let pool = init_db();
        use crate::core::config::state::AppState;
        AppState::new(pool)
    };


    // // auth
    // let auth_service: Arc<dyn IAuthService> = Arc::new(di_container.auth_service);
    // let auth_service_data: Data<dyn IAuthService> = Data::from(auth_service);
    //
    // // user
    // let user_service: Arc<dyn IUserService> = Arc::new(di_container.user_service);
    // let user_service_data: Data<dyn IUserService> = Data::from(user_service);


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
            .app_data(actix_web::web::Data::new(state.clone()))
            .configure(config_services)
    })
        .bind(&app_url)?
        .run()
        .await
}