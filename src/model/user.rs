use serde::{Serialize, Deserialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub pass: String,
    pub roles: String,
}

impl<'c> FromRow<'c, MySqlRow> for User {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(User { 
            id: row.get(0), 
            email: row.get(1), 
            pass: row.get(2), 
            roles: row.get(3),
        })
    }
}