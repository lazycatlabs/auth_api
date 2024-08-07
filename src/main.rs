use std::env;

use dotenv::dotenv;

use crate::core::config::server::run;

mod core;
mod features;
mod schema;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();

    run().await
}
