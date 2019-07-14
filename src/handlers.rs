use actix_web::{web, Responder};
use crate::user::User;

pub fn get_user(params: web::Form<String>) -> impl Responder {
    User {id: params.0, github: "yes".to_string()}
}

/*fn get_all_users(obj: web::Path<MyObj>) -> Result<HttpResponse> {
    let my_obj = MyObj {
        name: obj.name.to_string(),
    };
    let test = vec![my_obj; 3];
    Ok(HttpResponse::Ok().json(test))
}

fn create_user(obj: web::Path<MyObj>) -> Result<HttpResponse> {
    let my_obj = MyObj {
        name: obj.name.to_string(),
    };
    let test = vec![my_obj; 3];
    Ok(HttpResponse::Ok().json(test))
}


fn update_user(obj: web::Path<MyObj>) -> Result<HttpResponse> {
    let my_obj = MyObj {
        name: obj.name.to_string(),
    };
    let test = vec![my_obj; 3];
    Ok(HttpResponse::Ok().json(test))
}

fn delete_user(obj: web::Path<MyObj>) -> Result<HttpResponse> {
    let my_obj = MyObj {
        name: obj.name.to_string(),
    };
    let test = vec![my_obj; 3];
    Ok(HttpResponse::Ok().json(test))
}*/

#[cfg(test)]
mod handlers_tests {
    use super::*;

    mod get_user {
        use actix_web::dev::Service;
        use actix_web::{test, web, App};

        #[test]
        fn test_index_get() {
            let mut app = test::init_service(App::new().route("/", web::get().to(index)));
            let req = test::TestRequest::get().uri("/").to_request();
            let resp = test::block_on(app.call(req)).unwrap();

            assert!(resp.status().is_success());
        }

        #[test]
        fn test_index_post() {
            let mut app = test::init_service(App::new().route("/", web::get().to(index)));
            let req = test::TestRequest::post().uri("/").to_request();
            let resp = test::block_on(app.call(req)).unwrap();

            assert!(resp.status().is_client_error());
        }

        #[test]
        fn test_index_ok() {
            let req = test::TestRequest::with_header("content-type", "text/plain")
                .to_http_request();

            let resp = test::block_on(index(req)).unwrap();
            assert_eq!(resp.status(), http::StatusCode::OK);
        }

        #[test]
        fn test_index_not_ok() {
            let req = test::TestRequest::default().to_http_request();
            let resp = test::block_on(index(req)).unwrap();
            assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
        }
    }
}