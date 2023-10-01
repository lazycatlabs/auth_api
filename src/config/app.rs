use actix_web::web;

use crate::api::*;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    // Configuring routes
    cfg.service(
        web::scope("/api")
            .service(web::resource("/health_checker")
                .route(web::get().to(health_checker::health_checker)))
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
            ).service(
            web::resource("/profile")
                .route(web::get().to(account_controller::profile))
                .route(web::put().to(account_controller::update_user))
                .route(web::delete().to(account_controller::delete_user)))
    );
}