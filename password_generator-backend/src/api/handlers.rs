use crate::utils::generate_password::create_pass;
use crate::api::models::Selections;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn empty() -> impl Responder {
    let password: String = create_pass(30, true, true);
    HttpResponse::Ok().json(password)
}

#[get("/config")]
async fn config(params: web::Query<Selections>) -> impl Responder {
    handle_selections(params).await
}

async fn handle_selections(params: web::Query<Selections>) -> impl Responder {
    let length = params.length.unwrap_or(16) as u32;
    let put_numbers = params.put_numbers.unwrap_or(false);
    let put_symbols = params.put_symbols.unwrap_or(false);

    let password: String = create_pass(length, put_numbers, put_symbols);

    HttpResponse::Ok().json(password)
}