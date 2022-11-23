use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CityDto {
    pub id: i64,
    pub name: String,
}