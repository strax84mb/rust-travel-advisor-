use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow,
    Row,
    mysql::MySqlRow
};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub pass: String,
    pub roles: Vec<String>,
}

impl<'c> FromRow<'c, MySqlRow> for User {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        let id: i64 = match row.try_get(0) {
            Ok(id) => id,
            Err(err) => return Err(err),
        };
        let email: String = match row.try_get(1) {
            Ok(v) => v,
            Err(err) => return Err(err),
        };
        let pass: String = match row.try_get(2) {
            Ok(v) => v,
            Err(err) => return Err(err),
        };
        let roles_string: String = match row.try_get(3) {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        let roles: Vec<String> = roles_string.split(',').map(|s| s.to_string()).collect();

        Ok(User { 
            id: id, 
            email: email, 
            pass: pass, 
            roles: roles,
        })
    }
}