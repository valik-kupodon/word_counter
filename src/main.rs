mod database;
mod error;
mod schemas;
mod services;
mod urls;

use crate::urls::{get_count_of_word, parse_text, index};
use sqlx::postgres::PgPoolOptions;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let server_host = std::env::var("SERVER_HOST").expect("HOST not set");
    let server_port = std::env::var("SERVER_PORT").expect("PORT not set");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("src/migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");
    log::info!("Server started at http://{server_host}:{server_port}");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(get_count_of_word)
            .service(parse_text)
            .service(index)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(format!("{server_host}:{server_port}"))?
    .run()
    .await
}
