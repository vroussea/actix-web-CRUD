use actix_web::{web, App, HttpServer};
use crate::handlers::*;

mod handlers;
mod user;

pub fn main() {
    HttpServer::new(||
        App::new().route(r"/a/{name}", web::get().to(get_user)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}