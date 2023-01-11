use std::sync::Mutex;

use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    AppState,
    storage::airports, util::app_errors::Reason,
};
use super::{
    dtos::AirportDto,
    validations::string_to_id,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_airports)
        .service(get_airport_by_id);
}

#[get("/v1/airports")]
async fn get_airports(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let data_ref = data.get_ref();
    let db_lock = data_ref.lock().unwrap();
    let db_pool = db_lock.db.as_ref().connections.as_ref();
    // load airports
    let result = match airports::get_all(db_pool).await {
        Ok(loaded) => loaded,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };
    let result: Vec<AirportDto> = result.iter().map(|a| AirportDto::from_model(a)).collect();

    match serde_json::to_string(&result) {
        Ok(val) => HttpResponse::Ok().body(val),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/v1/airports/{id}")]
async fn get_airport_by_id(id: web::Path<String>, data: web::Data<Mutex<AppState>>) -> impl Responder {
    // check param
    let id = match string_to_id(id.to_string()) {
        Ok(id) => id,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };
    // extract DB
    let data_ref = data.get_ref();
    let db_lock = data_ref.lock().unwrap();
    let db_pool = db_lock.db.as_ref().connections.as_ref();
    // load airport
    let result = match airports::get_by_id(id, db_pool).await {
        Ok(airport) => AirportDto::from_model(&airport),
        Err(err) if err.type_message(Reason::NotFound).is_some() => return HttpResponse::NotFound().finish(),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };
    // serialize
    match serde_json::to_string(&result) {
        Ok(val) => HttpResponse::Ok().body(val),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
