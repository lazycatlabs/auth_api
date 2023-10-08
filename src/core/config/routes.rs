use actix_web::web;

use crate::{
    features::{
        auth::auth_controller,
        user::user_controller,
    },
    utils::{
        handler::route_not_found,
        health_checker,
    },
};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    // Configuring routes
    cfg.service(
        web::scope("/api")
            .service(web::resource("/health_checker")
                .route(web::get().to(health_checker::health_checker)))
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/login")
                            .route(web::post().to(auth_controller::login)),
                    )
                    .service(
                        web::resource("/logout")
                            .route(web::post().to(auth_controller::logout)),
                    )
            )
            .service(
                web::resource("/user")
                    .route(web::post().to(user_controller::register))
                    .route(web::get().to(user_controller::get_user))
                //     .route(web::put().to(account_controller::update_user))
                //     .route(web::delete().to(account_controller::delete_user)))
            ))
        .default_service(web::route().to(route_not_found));
}