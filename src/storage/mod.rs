mod db_context;
mod airport;
mod city;
mod user;

pub type Database = db_context::Database;

pub use city::cities as cities;
pub use airport::airports as airports;
