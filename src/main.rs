pub mod data;
pub mod service;
pub mod enviroment;

extern crate dotenv;
use documents::documents_server::DocumentsServer;
use dotenv::dotenv;
use enviroment::Enviroment;
use tonic::transport::Server;

pub mod documents {
    tonic::include_proto!("documents");
}

#[tokio::main]
async fn main() {
    dotenv().expect("Please create '.env' file and complete like '.env.example'");

    let env = Enviroment::init();
    Server::builder()
        .add_service(DocumentsServer::new(env.service))
        .serve(env.addr)
        .await.expect("cant't run documents service");
}
