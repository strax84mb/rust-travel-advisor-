pub mod api {
    use actix_web::{post, web, Responder, HttpResponse};
    use serde::{Deserialize,Serialize};
    use crate::services::get_auth_service;
    use crate::services::AuthService;

    #[derive(Deserialize)]
    struct LoginRequest {
        username: String,
        password: String
    }

    #[derive(Serialize)]
    struct LoginResponse {
        id: i64,
        roles: Vec<String>,
        jwt: String
    }

    #[post("/v1/login")]
    async fn login(payload: web::Json<LoginRequest>) -> impl Responder {
        let service = get_auth_service();
        let message: String = service.login();
        HttpResponse::Ok().body(message)
    }
}