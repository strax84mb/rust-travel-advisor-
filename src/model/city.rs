use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct City {
    pub id: i64,
    pub name: String,
}