use std::sync::Mutex;

use actix_web::{get, web, Responder, HttpResponse};

use crate::AppState;
use super::dtos::CityDto;
 
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cities);
}

#[get("/v1/cities")]
async fn get_cities(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let data_ref = data.get_ref();
    let db_lock = data_ref.lock().unwrap();
    let db = db_lock.db.as_ref();
    
    let cities = db.get_cities().await;
    let cities = match cities {
        Ok(cities) => cities,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let dtos: Vec<CityDto> = cities.iter()
        .map(|c| CityDto {
            id: c.id,
            name: c.name.clone(),
        })
        .collect();

    match serde_json::to_string(&dtos) {
        Ok(json) => HttpResponse::Ok().body(json),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}