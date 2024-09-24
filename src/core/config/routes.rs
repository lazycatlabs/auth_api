use actix_web::web;

use crate::{
    features::{
        auth::routes::auth_scope, general::routes::general_scope, user::routes::user_scope,
    },
    utils::handler::route_not_found,
};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    // Configuring routes
    cfg.service(
        web::scope("/api")
            .service(general_scope())
            .service(auth_scope())
            .service(user_scope()),
    )
    .default_service(web::route().to(route_not_found));
}
