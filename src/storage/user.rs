use sqlx::FromRow;

use crate::{
    util::app_errors::Error,
    model::User
};
use super::db_context::Database;

impl Database {
    pub async fn get_user(&self, id: i64) -> Result<User, Error> {
        let result = sqlx::query("SELECT id, email, pass, roles FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(self.connections.as_ref())
            .await;
        match result {
            Ok(row) => {
                match User::from_row(&row) {
                    Ok(user) => Ok(user),
                    Err(err) => Err(Error::underlying(err.to_string())),
                }
            },
            Err(sqlx::Error::RowNotFound) => Err(Error::not_found()),
            Err(err) => Err(Error::underlying(err.to_string())),
        }
    }

    pub async fn get_user_by_email_and_pass(&self, email: String, password: String) -> Result<User, Error> {
        let pass = md5::compute(password.as_bytes());
        let pass: String = pass.iter().map(|&q| q as char).collect();
        let result = sqlx::query("SELECT id, email, pass, roles FROM users WHERE email = ? AND pass = ?")
            .bind(email)
            .bind(pass)
            .fetch_one(self.connections.as_ref())
            .await;
        match result {
            Ok(row) => {
                match User::from_row(&row) {
                    Ok(user) => Ok(user),
                    Err(err) => Err(Error::underlying(err.to_string())),
                }
            },
            Err(sqlx::Error::RowNotFound) => Err(Error::not_found()),
            Err(err) => Err(Error::underlying(err.to_string())),
        }
    }
}