use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod orders;
pub mod products;
pub mod stock;

pub trait Item<T> {
    fn insert();

    fn delete();
}

pub fn get_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DB path not set");

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}
