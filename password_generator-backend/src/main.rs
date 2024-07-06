mod api;
mod utils;

use actix_web::{ App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(api::routes::paths)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}