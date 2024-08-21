use actix_web::web;

use crate::{
    features::{auth::auth_controller, general::general_controller, user::user_controller},
    utils::handler::route_not_found,
};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    // Configuring routes
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/general")
                    .service(
                        web::resource("/send_email")
                            .route(web::post().to(general_controller::mail_sender_controller)),
                    )
                    .service(
                        web::resource("/health_checker")
                            .route(web::get().to(general_controller::health_checker_controller)),
                    ),
            )
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/general")
                            .route(web::post().to(auth_controller::general_token_controller)),
                    )
                    .service(
                        web::resource("/login")
                            .route(web::post().to(auth_controller::login_contoller)),
                    )
                    .service(
                        web::resource("/logout")
                            .route(web::post().to(auth_controller::logout_controller)),
                    )
                    .service(
                        web::resource("/session")
                            .route(web::get().to(auth_controller::login_session_controller)),
                    )
                    .service(
                        web::resource("/password")
                            .route(web::put().to(auth_controller::update_password_controller)),
                    ),
            )
            .service(
                web::scope("/user")
                    .service(
                        web::resource("")
                            .route(web::post().to(user_controller::register_controller))
                            .route(web::get().to(user_controller::get_user))
                            .route(web::put().to(user_controller::update_user_controller))
                            .route(web::delete().to(user_controller::delete_user_controller)),
                    )
                    .service(
                        web::resource("/all")
                            .route(web::get().to(user_controller::users_controller)),
                    ),
            ),
    )
    .default_service(web::route().to(route_not_found));
}
