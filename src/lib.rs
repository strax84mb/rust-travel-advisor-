#[path = "storage/mod.rs"]
mod storage;
#[path = "api/mod.rs"]
mod api;
#[path = "services/mod.rs"]
mod services;

use actix_web::{web, App, HttpServer};
use api::hello_world;

fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(hello_world)
    );
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(init)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
