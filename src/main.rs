mod database;
mod error;
mod schemas;
mod services;
mod urls;

use crate::urls::{get_count_of_word, parse_text, index};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let server_host = std::env::var("SERVER_HOST").expect("HOST hot set");
    let server_port = std::env::var("SERVER_PORT").expect("HOST hot set");
    log::info!("Server started at http://{server_host}:{server_port}");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(get_count_of_word)
            .service(parse_text)
            .service(index)
    })
    .bind(format!("{server_host}:{server_port}"))?
    .run()
    .await
}
