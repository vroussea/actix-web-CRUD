use crate::database::*;
use actix_web::{web, Error, HttpResponse, error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use futures::Future;
use diesel::result::Error::AlreadyInTransaction;
use crate::handlers_structs::*;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_user(
    item: web::Json<GetUserStruct>,
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
    item: web::Json<CreateUserStruct>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let clone_item = item.into_inner().clone();
    web::block(move || create(clone_item.name, clone_item.github_name, clone_item.github_password, clone_item.password, pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn update_user(
    item: web::Json<UpdateUserStruct>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let clone_item = item.into_inner().clone();
    web::block(move || update(clone_item.name, clone_item.github_name, clone_item.github_password, clone_item.password, clone_item.id, pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {match err { error::BlockingError::Error(e) => if e == AlreadyInTransaction {Ok(HttpResponse::Unauthorized().into())} else { Ok(HttpResponse::InternalServerError().into()) }, error::BlockingError::Canceled => Ok(HttpResponse::InternalServerError().into()), }},
    })
}

pub fn delete_user(
    item: web::Json<DeleteUserStruct>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let clone_item = item.into_inner().clone();
    web::block(move || delete(clone_item.password, clone_item.id, pool)).then(|res| match res {
        Ok(_) => Ok(HttpResponse::Ok().into()),
        Err(err) => {match err { error::BlockingError::Error(e) => if e == AlreadyInTransaction {Ok(HttpResponse::Unauthorized().into())} else { Ok(HttpResponse::InternalServerError().into()) }, error::BlockingError::Canceled => Ok(HttpResponse::InternalServerError().into()), }},
    })
}

/*pub fn get_user_github(
    item: web::Json<MyUser>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    System::new("test").block_on(lazy(|| {
        awc::Client::new()
            .get("https://www.rust-lang.org/") // <- Create request builder
            .header("User-Agent", "Actix-web")
            .send() // <- Send http request
            .from_err()
            .and_then(|mut response| {
                // <- server http response
                println!("Response: {:?}", response);

                // read response body
                response
            })
    }));

    /*let sys = actix::System::new("test");

    actix::Arbiter::handle().spawn({
        client::get("http://www.rust-lang.org")   // <- Create request builder
            .header("User-Agent", "Actix-web")
            .finish().unwrap()
            .send()                               // <- Send http request
            .map_err(|_| ())
            .and_then(|response| {                // <- server http response
                println!("Response: {:?}", response);
                response
            })
    });

    sys.run();*/
    Ok(HttpResponse::InternalServerError().into())
    /*web::block(move || query(item.into_inner().name, pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })*/
}*/

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
