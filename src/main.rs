pub mod data;
pub mod service;

pub mod enviroment;

extern crate dotenv;
use dotenv::dotenv;

pub mod documents {
    tonic::include_proto!("documents");
}

#[tokio::main]
async fn main() {
    dotenv().expect("Please create '.env' file and complete like '.env.example'");
    println!("Hello, world!");
}
