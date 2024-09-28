use actix_web::http::header::HeaderValue;

pub fn token_extractor(auth: &str) -> String {
    let bearer_str = auth.split(' ').collect::<Vec<&str>>();
    let token_prefix = bearer_str[1].split('.').collect::<Vec<&str>>();

    token_prefix[1..].join(".")
}

pub fn is_auth_header_valid(auth_header: &HeaderValue) -> bool {
    if let Ok(auth_str) = auth_header.to_str() {
        return auth_str.starts_with("bearer") || auth_str.starts_with("Bearer");
    }
    false
}
