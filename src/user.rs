use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub github: String,
}

impl Responder for User {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}