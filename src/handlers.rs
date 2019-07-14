use actix_web::{web, Responder, http};
use crate::user::User;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

pub fn get_user(request: HttpRequest) -> HttpResponse {
    let id = match request.match_info().get("id") {
        Some(id) => id,
        None => return HttpResponse::BadRequest().into(),
    };
    let user = User {id: id.to_string(), username: "user".to_string(), github: Some("yes".to_string())};
    HttpResponse::Ok().json(user)
}

/*fn get_all_users(obj: web::Path<MyObj>) -> Result<HttpResponse> {
    let my_obj = MyObj {
        name: obj.name.to_string(),
    };
    let test = vec![my_obj; 3];
    Ok(HttpResponse::Ok().json(test))
}*/

fn create_user(request: HttpRequest) -> HttpResponse {
    let username = match request.match_info().get("username") {
        Some(username) => username,
        None => return HttpResponse::BadRequest().into(),
    };
    let github = match request.match_info().get("github") {
        Some(github) => Some(github.to_string()),
        None => None,
    };
    let user = User {id: "1".to_string(), username: username.to_string(), github: github};
    HttpResponse::Ok().json(user)
}


/*fn update_user(obj: web::Path<MyObj>) -> Result<HttpResponse> {
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
    mod get_user {
        use super::super::*;
        use actix_web::dev::Service;
        use actix_web::{test, web, App, http};

        #[test]
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
        }
    }

    mod create_user {
        use super::super::*;
        use actix_web::dev::Service;
        use actix_web::{test, web, App, http};

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

        #[test]
        fn test_ok() {
            let req = test::TestRequest::default().param("username", "1234")
                .to_http_request();

            let resp = test::block_on(create_user(req)).unwrap();
            assert_eq!(resp.status(), http::StatusCode::OK);
        }

        #[test]
        fn test_not_ok() {
            let req = test::TestRequest::default().to_http_request();
            let resp = test::block_on(create_user(req)).unwrap();
            assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
        }
    }
}