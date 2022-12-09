use std::sync::Mutex;

use actix_web::{get, post, web, Responder, HttpResponse, HttpRequest};

use crate::{
    AppState,
    util::app_errors::Reason::NotFound
};
use super::{
    auth::validate_request,
    dtos::CityDto,
    validations::string_to_id
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cities)
        .service(get_city_by_id)
        .service(upload_cities);
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

#[post("/v1/cities")]
async fn upload_cities(req: HttpRequest, payload: web::Bytes, data: web::Data<Mutex<AppState>>) -> impl Responder {
    match validate_request(&req, vec!["admin".to_string()]) {
        Err(err) => return HttpResponse::Unauthorized().body(err.to_string()),
        _ => (),
    }

    let data_ref = data.get_ref();
    let db_lock = data_ref.lock().unwrap();
    let db = db_lock.db.as_ref();

    let mut saved: i16 = 0;

    let payload_vec = payload.to_vec();
    let mut csv_reader = csv::Reader::from_reader(payload_vec.as_slice());
    for record in csv_reader.records() {
        let record = match record {
            Ok(r) => r,
            Err(err) => return HttpResponse::BadRequest().body(format!("malformed CSV: {}", err.to_string())),
        };

        let name = record[0].to_string();
        match db.save_city(name.clone()).await {
            Err(err) => return HttpResponse::InternalServerError().body(format!("failed to save city {}: {}", name, err.to_string())),
            _ => saved += 1,
        };
    }

    HttpResponse::Ok().body(format!("successfuly saved {}", saved))
}