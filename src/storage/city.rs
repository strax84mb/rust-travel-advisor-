use sqlx::FromRow;

use crate::model::City;
use crate::util::app_errors::Error;
use super::db_context::Database;

impl Database {
    pub async fn get_cities(&self) -> Result<Vec<City>, Error> {
        match sqlx::query("SELECT id, name FROM cities").fetch_all(self.connections.as_ref()).await {
            Ok(rows) => {
                let result: Result<Vec<City>, sqlx::Error> = rows.iter().map(|row| City::from_row(row)).collect();
                match result {
                    Ok(v) => Ok(v),
                    Err(err) => Err(Error::underlying(err.to_string()))
                }
            },
            Err(err) => Err(Error::underlying(err.to_string()))
        }
    }

    pub async fn get_city_by_id(&self, id: i64) -> Result<City, Error> {
        let result = sqlx::query("SELECT id, name FROM cities WHERE id = $1")
            .bind(id)
            .fetch_one(self.connections.as_ref())
            .await;
        match result {
            Ok(row) => {
                match City::from_row(&row) {
                    Ok(city) => Ok(city),
                    Err(err) => Err(Error::underlying(err.to_string())),
                }
            },
            Err(sqlx::Error::RowNotFound) => Err(Error::not_found()),
            Err(err) => Err(Error::underlying(err.to_string())),
        }
    }

    pub async fn save_city(&self, name: String) -> Result<City, Error> {
        let result = sqlx::query("INSERT INTO cities (name) VALUES ($1)")
            .bind(name.clone())
            .execute(self.connections.as_ref())
            .await;
        match result {
            Ok(row) => {
                if row.rows_affected() == 0 {
                    return Err(Error::underlying("no rows inserted".to_string()));
                }

                let id = row.last_insert_id() as i64;

                Ok(City {
                    id: id,
                    name: name,
                })
            },
            Err(err) => Err(Error::underlying(err.to_string())),
        }
    }
}
