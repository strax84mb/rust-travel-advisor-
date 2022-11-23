use sqlx::FromRow;

use crate::model::City;
use super::db_context::Database;

impl Database {
    pub async fn get_cities(&self) -> Result<Vec<City>, String> {
        match sqlx::query("SELECT id, name FROM cities").fetch_all(self.connections.as_ref()).await {
            Ok(rows) => {
                let result: Result<Vec<City>, sqlx::Error> = rows.iter().map(|row| City::from_row(row)).collect();
                match result {
                    Ok(v) => Ok(v),
                    Err(err) => Err(err.to_string())
                }
            },
            Err(err) => Err(err.to_string())
        }
    }
}