use actix_web::web;

use super::general_controller;

pub fn general_scope() -> actix_web::Scope {
  web::scope("/general")
  .service(
      web::resource("/send_email")
          .route(web::post().to(general_controller::mail_sender_controller)),
  )
  .service(
      web::resource("/health_checker")
          .route(web::get().to(general_controller::health_checker_controller)),
  )
}
