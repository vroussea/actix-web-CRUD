#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;

use crate::handlers::*;

mod handlers;
mod models;
mod database;
mod schema;

pub fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    /*HttpServer::new(move ||
        App::new().data(pool.clone()).route("/add/{username}", web::get().to_async(create_user)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();*/

        HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/add")
                    .data(
                        web::JsonConfig::default()
                            .limit(4096)
                            .error_handler(|err, _| {
                                error::InternalError::from_response(
                                    err,
                                    HttpResponse::Conflict().finish(),
                                )
                                    .into()
                            }),
                    )
                    .route(web::post().to_async(create_user)),
            )
            /*.service(web::resource("/add").route(web::post().to_async(index_add)))
            .service(web::resource("/add/{name}").route(web::get().to_async(add)))*/
        })
        .bind("127.0.0.1:8088")?
        .run()
}