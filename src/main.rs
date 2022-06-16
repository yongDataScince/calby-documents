pub mod data;
pub mod service;

pub mod documents {
    tonic::include_proto!("documents");
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
