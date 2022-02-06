use actix_web::{get, web, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
async fn index() -> impl Responder {
    super::schemas::index::StatusResponse { status: "Ok" }
}
