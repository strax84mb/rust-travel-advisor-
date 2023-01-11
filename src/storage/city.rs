pub mod cities {
    use sqlx::{FromRow, MySql, Pool};

    use crate::{
        model::City,
        util::app_errors::Error
    };
    
    pub async fn get_all(pool: &Pool<MySql>) -> Result<Vec<City>, Error> {
        match sqlx::query("SELECT id, name FROM cities").fetch_all(pool).await {
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

    pub async fn get_by_id(id: i64, pool: &Pool<MySql>) -> Result<City, Error> {
        let result = sqlx::query("SELECT id, name FROM cities WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
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

    pub async fn new(name: String, pool: &Pool<MySql>) -> Result<City, Error> {
        let result = sqlx::query("INSERT INTO cities (name) VALUES (?)")
            .bind(name.clone())
            .execute(pool)
            .await;
        match result {
            Ok(row) => {
                if row.rows_affected() == 0 {
                    return Err(Error::underlying("no rows inserted".to_string()));
                }

                let id = row.last_insert_id() as i64;

                Ok(City::new(id, name))
            },
            Err(err) => Err(Error::underlying(err.to_string())),
        }
    }
}