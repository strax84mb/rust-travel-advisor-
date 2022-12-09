use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow,
    Row,
    mysql::MySqlRow
};

#[derive(Serialize, Deserialize, Clone)]
pub struct City {
    pub id: i64,
    pub name: String,
}

impl<'c> FromRow<'c, MySqlRow> for City {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        let id: i64 = match row.try_get(0) {
            Ok(i) => i,
            Err(err) => return Err(err),
        };

        let name: String = match row.try_get(1) {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        Ok(City {
            id: id,
            name: name,
        })
    }
}