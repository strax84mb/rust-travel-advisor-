pub mod api {
    use actix_web::{get, Responder, HttpResponse};
    
    #[get("/v1/hello")]
    async fn hello_world() -> impl Responder {
        HttpResponse::Ok().body("World!")
    }
    
}