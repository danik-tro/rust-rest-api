use actix_web::{get, web, HttpResponse, Responder};


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
