use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CityDto {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDto {
    pub id: i64,
    pub email: String,
    pub roles: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pass: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub id: i64,
    pub token: String,
}