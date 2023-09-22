use actix_web::{get, HttpResponse, HttpServer, Responder};

use crate::models::response::{Diagnostic, ResponseBodyNoData};

mod models;
mod config;

#[get("/api/healthchecker")]
async fn healthchecker() -> impl Responder {
    const MESSAGE: &str = "Hello, Lzyct";

    HttpResponse::Ok().json(ResponseBodyNoData::new(Diagnostic::new("200", MESSAGE)))
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let (server, port) = ("127.0.0.1", 8000);

    println!("🚀Server started {}:{}", server, port);

    HttpServer::new(|| {
        actix_web::App::new()
            .service(healthchecker)
    }).bind((server, port))?
        .run()
        .await
}
