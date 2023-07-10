mod error;
mod schemas;
mod services;
mod urls;

use crate::urls::{get_count_of_word, parse_text};
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(|| App::new().service(get_count_of_word).service(parse_text))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
