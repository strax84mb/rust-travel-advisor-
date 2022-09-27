pub mod db {
    use diesel::prelude::*;
    use diesel::mysql::MysqlConnection;
    use std::env;
    //use dotenv::dotenv;

    pub fn establish_connection() -> MysqlConnection {
        //dotenv().ok();
 
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        MysqlConnection::establish(database_url.as_str()).expect("failed to connect to DB")
    }
}