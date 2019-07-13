fn get_user(obj: web::Path<MyObj>) -> Result<HttpResponse> {
    let my_obj = MyObj {
        name: obj.name.to_string(),
    };
    let test = vec![my_obj; 3];
    Ok(HttpResponse::Ok().json(test))
}

fn get_all_users(obj: web::Path<MyObj>) -> Result<HttpResponse> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

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