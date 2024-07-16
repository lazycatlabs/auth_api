use actix_cors::Cors;
use actix_web::http::header;

pub fn cors() -> Cors {
    Cors::default()
        .send_wildcard()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600)
}
