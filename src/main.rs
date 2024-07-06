mod password;

use password::create_pass;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Selections {
    length: Option<usize>,
    put_numbers: Option<bool>,
    put_symbols: Option<bool>,
}

async fn handle_selections(params: web::Query<Selections>) -> impl Responder {
    let length = params.length.unwrap_or(16) as u32;
    let put_numbers = params.put_numbers.unwrap_or(false);
    let put_symbols = params.put_symbols.unwrap_or(false);

    let password: String = create_pass(length, put_numbers, put_symbols);

    HttpResponse::Ok().json(password)
}

#[get("/config")]
async fn config(params: web::Query<Selections>) -> impl Responder {
    handle_selections(params).await
}

#[get("/")]
async fn hello() -> impl Responder {
    let password: String = create_pass(30, true, true);
    HttpResponse::Ok().body(password)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}