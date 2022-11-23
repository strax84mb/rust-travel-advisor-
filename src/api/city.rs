use std::sync::Mutex;

use actix_web::{get, web, Responder, HttpResponse};
use sqlx::Row;
use crate::AppState;
use super::dtos::CityDto;
 
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cities);
}

#[get("/v1/cities")]
async fn get_cities(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let data_ref = data.get_ref();
    let db_lock = data_ref.lock().unwrap();
    let db = db_lock.db.connections.as_ref();

    match sqlx::query("SELECT id, name FROM cities").fetch_all(db).await {
        Ok(res) => {
            let final_vec: Result<Vec<CityDto>, sqlx::Error> = res.iter()
                .map(|r| {
                    let id: i64 = match r.try_get(0) {
                        Ok(i) => i,
                        Err(err) => {
                            return Err(err);
                        },
                    };

                    let name: String = match r.try_get(1) {
                        Ok(name) => name,
                        Err(err) => return Err(err),
                    };

                    Ok(CityDto{
                        id: id,
                        name: name,
                    })
                })
                .collect();
            match final_vec {
                Ok(res) => {
                    match serde_json::to_string(&res) {
                        Ok(json) => return HttpResponse::Ok().body(json),
                        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
                    };
                },
                Err(err) => {
                    return HttpResponse::InternalServerError().body(err.to_string());
                },
            }
        },
        Err(err) => {
            return HttpResponse::InternalServerError().body(err.to_string());
        },
    }
}