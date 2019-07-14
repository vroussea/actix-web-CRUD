use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

mod handlers;

#[derive(Serialize, Deserialize, Clone)]
struct MyObj {
    name: String,
}

fn index(obj: web::Path<MyObj>) -> Result<HttpResponse> {
    let my_obj = MyObj {
        name: obj.name.to_string(),
    };
    let test = vec![my_obj; 3];
    Ok(HttpResponse::Ok().json(test))
}

pub fn main() {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().route(r"/a/{name}", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}