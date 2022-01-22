use diesel::prelude::*;
use diesel::MysqlConnection;
use dotenv::dotenv;
use std::env;

/// データベースコネクション
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("env error");

    MysqlConnection::establish(&database_url).expect("database connection error")
}
