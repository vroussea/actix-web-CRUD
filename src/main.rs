#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;

use crate::configure::*;
use crate::requests::*;

mod configure;
mod database;
mod handlers;
mod models;
mod schema;
mod requests;
mod handlers_structs;

pub fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv::dotenv().ok();

    if let Err(e) = get_user_repositories() {
        println!("something went wrong : {}", e);
    }

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(add)
            .configure(get_one)
            .configure(get_all)
            .configure(update)
            .configure(delete)
            //.configure(get_github)
    })
    .bind("127.0.0.1:8088")?
    .run()
}
