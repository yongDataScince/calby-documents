use diesel::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod model;
pub mod schema;
use crate::enviroment::not_provided;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect(&not_provided("DATABASE_URL"));

  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}