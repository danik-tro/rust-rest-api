use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use std::future::{ready, Ready};

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: &'static str,
}

impl Responder for StatusResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
