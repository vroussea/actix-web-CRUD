use crate::database::*;
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use futures::Future;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_user(
    item: web::Json<MyUser>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || query(item.into_inner().name, pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn get_users(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || query_all(pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn create_user(
    item: web::Json<MyUser>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || create(item.into_inner().name, pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn update_user(
    item: web::Json<MyUser>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let clone_item = item.into_inner().clone();
    web::block(move || update(clone_item.name, clone_item.id, pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn delete_user(
    item: web::Json<MyUser>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || delete(item.into_inner().id, pool)).then(|res| match res {
        Ok(_) => Ok(HttpResponse::Ok().into()),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

#[cfg(test)]
mod handlers_tests {
    mod get_user {
        use super::super::*;
        use actix_web::dev::Service;
        use actix_web::{http, test, web, App};

        /*#[test]
        fn test_ok() {
            let req = test::TestRequest::default().param("id", "1234")
                .to_http_request();

            let resp = test::block_on(get_user(req)).unwrap();
            assert_eq!(resp.status(), http::StatusCode::OK);
        }

        #[test]
        fn test_not_ok() {
            let req = test::TestRequest::default().to_http_request();
            let resp = test::block_on(get_user(req)).unwrap();
            assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
        }*/
    }

    mod create_user {
        use super::super::*;
        use actix_web::dev::Service;
        use actix_web::{http, test, web, App};

        /*#[test]
        fn test_correct_username() {
            let req = test::TestRequest::default().param("username", "1234")
                .to_http_request();

            let resp = test::block_on(create_user(req)).unwrap();
            let result = User::from(test::read_body(resp));
            assert_eq!(result.username, "1234".to_string());
        }

        #[test]
        fn test_has_github() {
            let req = test::TestRequest::default().param("username", "1234")
                .to_http_request();

            let resp = test::block_on(create_user(req)).unwrap();
            assert_eq!(resp.status(), http::StatusCode::OK);
        }*/

        /*#[test]
        fn test_ok() {
            let req = test::TestRequest::default().param("username", "1234")
                .to_http_request();

            let resp = test::block_on(create_user(req)).unwrap();
            assert_eq!(resp.status(), http::StatusCode::OK);
        }

        #[test]
        fn test_not_ok() {
            std::env::set_var("RUST_LOG", "actix_web=info");
            env_logger::init();

            dotenv::dotenv().ok();

            let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
            let manager = ConnectionManager::<SqliteConnection>::new(connspec);
            let pool = r2d2::Pool::builder()
                .build(manager)
                .expect("Failed to create pool.");

            let req = test::TestRequest::default().to_http_request();
            let resp = test::block_on(create_user(req)).unwrap();
            assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
        }*/
    }
}
