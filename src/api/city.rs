use std::sync::Mutex;

use actix_web::{get, web, Responder, HttpResponse};

use crate::AppState;
use crate::util::app_errors::Reason::NotFound;
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

use super::validations::string_to_id;

#[get("/v1/cities/{id}")]
async fn  get_city_by_id(
    id: web::Path<String>,
    data: web::Data<Mutex<AppState>>,
) -> impl Responder {
    let data_ref = data.get_ref();
    let db_lock = data_ref.lock().unwrap();
    let db = db_lock.db.as_ref();
    // get id
    let city_id = match string_to_id(id.to_string()) {
        Ok(v) => v,
        Err(err) => return HttpResponse::BadRequest().body(format!("failed to parse city ID: {}", err)),
    };
    // load city
    let city = match db.get_city_by_id(city_id).await {
        Ok(c) => CityDto{
            id: c.id,
            name: c.name,
        },
        Err(err) if err.type_message(NotFound).is_some() => return HttpResponse::NotFound().body(format!("city not found")),
        Err(err) => return HttpResponse::InternalServerError().body(format!("failed to load city: {}", err.to_string())),
    };

    match serde_json::to_string(&city) {
        Ok(json) => HttpResponse::Ok().body(json),
        Err(err) => HttpResponse::InternalServerError().body(format!("failed to load city: {}", err.to_string())),
    }
}