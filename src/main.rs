pub mod data;
pub mod enviroment;

extern crate dotenv;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().expect("Please create '.env' file and complete like '.env.example'");
    println!("Hello, world!");
}
