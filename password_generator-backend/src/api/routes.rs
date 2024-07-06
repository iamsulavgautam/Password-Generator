use actix_web::web;
use crate::api::handlers;

pub fn paths(cfg: &mut web::ServiceConfig) {
    cfg
        .service(handlers::empty)
        .service(handlers::config);
}