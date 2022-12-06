use actix_web::{get, web, Responder, HttpResponse};

 
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_world);
}

#[get("/v1/hello")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("World!")
}
