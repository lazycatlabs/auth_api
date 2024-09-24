use actix_web::web;

use super::user_controller;

pub fn user_scope() -> actix_web::Scope {
    web::scope("/user")
        .service(
            web::resource("")
                .route(web::post().to(user_controller::register_controller))
                .route(web::get().to(user_controller::get_user))
                .route(web::put().to(user_controller::update_user_controller))
                .route(web::delete().to(user_controller::delete_user_controller)),
        )
        .service(web::resource("/all").route(web::get().to(user_controller::users_controller)))
}
