use actix_web::web;

use crate::api::*;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    // Configuring routes
    cfg.service(
        web::scope("/api")
            .service(health_checker::health_checker)
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/signup")
                            .route(web::post().to(account_controller::signup)),
                    )
                    .service(
                        web::resource("/login")
                            .route(web::post().to(account_controller::login)),
                    )
                    .service(
                        web::resource("/logout")
                            .route(web::post().to(account_controller::logout)),
                    )
            )
    );
}