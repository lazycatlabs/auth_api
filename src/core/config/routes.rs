use actix_web::web;

use crate::{
    features::{auth::auth_controller, general::general_controller, user::user_controller},
    utils::{handler::route_not_found, health_checker},
};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    // Configuring routes
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/health_checker")
                    .route(web::get().to(health_checker::health_checker)),
            )
            .service(web::scope("/general").service(
                web::resource("/send_email").route(web::post().to(general_controller::test_email)),
            ))
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/general")
                            .route(web::post().to(auth_controller::general_token)),
                    )
                    .service(web::resource("/login").route(web::post().to(auth_controller::login)))
                    .service(
                        web::resource("/logout").route(web::post().to(auth_controller::logout)),
                    )
                    .service(
                        web::resource("/session")
                            .route(web::get().to(auth_controller::login_session)),
                    )
                    .service(
                        web::resource("/password")
                            .route(web::put().to(auth_controller::update_password)),
                    ),
            )
            .service(
                web::resource("/user")
                    .route(web::post().to(user_controller::register))
                    .route(web::get().to(user_controller::get_user))
                    .route(web::put().to(user_controller::update_user))
                    .route(web::delete().to(user_controller::delete_user)),
            ),
    )
    .default_service(web::route().to(route_not_found));
}
