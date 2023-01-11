mod config;
mod lib;
pub mod api;
pub mod model;
pub mod services;
pub mod storage;
pub mod util;

use std::process::exit;
use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer};
use actix_web::web::Data;

use crate::config::Config;
use crate::api::{init_hello, init_city, init_user, init_airport};
use crate::storage::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let config_file: &'static str = "config.json";
    let config = Config::from_file(config_file);
    println!("Using configuration file from {0}", config_file);

    let database = Database::new(config.get_database_url().clone());
    let database = match database.await {
        Ok(db) => db,
        Err(err) => {
            println!("Failed to init DB: {}", err.as_str());
            exit(1);
        },
    };

    let app_state = AppState{
        db: Arc::new(database),
    };

    let data = Data::new(Mutex::new(app_state));

    let app = HttpServer::new(move || {
        App::new()
            /*.wrap_fn(|req, srv| {
                let q = req.into_response(HttpResponse::Unauthorized().finish());
                //let w = Either::Left(q);
                async move {
                    Ok(q)
                }
            })*/
            .app_data(Data::clone(&data))
            .configure(init_hello)
            .configure(init_city)
            .configure(init_user)
            .configure(init_airport)
        }
    ).bind(config.get_app_url())?;

//    lib::run().expect("failed to run");
    app.run().await
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}