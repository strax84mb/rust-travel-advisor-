use std::sync::Mutex;

use actix_web::{post, web, Responder, HttpResponse};

use crate::AppState;
use super::{
    auth::create_jwt,
    dtos::{LoginRequest, LoginResponse}
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}

#[post("/v1/login")]
async fn login(payload: web::Json<LoginRequest>, data: web::Data<Mutex<AppState>>) -> impl Responder {
    let data_ref = data.get_ref();
    let db_lock = data_ref.lock().unwrap();
    let db = db_lock.db.as_ref();

    let request = payload.into_inner();
    let user = db.get_user_by_email_and_pass(
        request.email.clone(), 
        request.pass.clone(),
    ).await;
    let user = match user {
        Ok(u) => u,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let token = create_jwt(request.email, user.roles);
    let token = match token {
        Ok(t) => t,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let response = LoginResponse {
        id: user.id.clone(),
        token: token,
    };

    match serde_json::to_string(&response) {
        Ok(json) => HttpResponse::Ok().body(json),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
