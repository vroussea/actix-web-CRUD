use actix_web::{web, App, HttpServer};
use crate::handlers::*;
//use rustbreak::Database;

mod handlers;
mod user;

pub fn main() {
    /*let db = Database::open("my_contacts").unwrap();

    db.insert("Lapfox", "lapfoxtrax.com").unwrap();
    db.insert("Rust", "github.com/rust-lang/rust").unwrap();

    // we need to be explicit about the kind of type we want as println! is
    // generic
    let rust : String = db.retrieve("Rust").unwrap();
    println!("You can find Rust at: {}", rust);
    db.save();*/

    HttpServer::new(||
        App::new().route("/user/{id}", web::get().to(get_user)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}