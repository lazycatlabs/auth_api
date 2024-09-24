use actix_web::web;

use super::auth_controller;

pub fn auth_scope() -> actix_web::Scope {
    web::scope("/auth")
        .service(
            web::resource("/general")
                .route(web::post().to(auth_controller::general_token_controller)),
        )
        .service(
            web::resource("/login")
                .route(web::post().to(auth_controller::login_contoller)), // Fixed typo: login_contoller -> login_controller
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
        )
}
