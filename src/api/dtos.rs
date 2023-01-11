use serde::{Serialize, Deserialize};

use crate::model::Airport;

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

#[derive(Serialize, Deserialize)]
pub struct AirportDto {
    pub id: i64,
    pub city_id: i64,
    pub name: String,
}

impl AirportDto {
    pub fn from_model(a: &Airport) -> AirportDto {
        AirportDto {
            id: a.id.clone(),
            city_id: a.city_id.clone(),
            name: a.name.clone(),
        }
    }

    pub fn to_model(&self) -> Airport {
        Airport {
            id: self.id.clone(),
            city_id: self.city_id.clone(),
            name: self.name.clone(),
        }
    }
}