pub mod data;
pub mod service;
pub mod enviroment;
pub mod psql_handler;
pub mod utils;
pub mod documents {
    tonic::include_proto!("documents");
}

extern crate dotenv;
use documents::documents_server::DocumentsServer;
use dotenv::dotenv;
use enviroment::Enviroment;
use tonic::transport::Server;
use hyper::service::make_service_fn;

use crate::psql_handler::{Db, db_setup};


pub async fn shutdown() {
  tokio::signal::ctrl_c()
    .await
    .expect("cant't detect Ctrl+C");
}

#[tokio::main]
async fn main() {
    dotenv().expect("Please create '.env' file and complete like '.env.example'");

    let manager = bb8_postgres::PostgresConnectionManager::new_from_stringlike(
        "host=localhost password=docpass dbname=docdb user=postgres connect_timeout=10 keepalives=0",
        tokio_postgres::NoTls
    ).unwrap();
    let pool = bb8::Pool::builder().max_size(15).build(manager).await.unwrap();
    let db = Db::new(pool);
    db_setup(&db).await.expect("cant setup databse");

    let env = Enviroment::init(db);
    let service = Server::builder()
        .add_service(DocumentsServer::new(env.service.to_owned()))
        .into_service();

    let make_service = make_service_fn(move |_conn: &hyper::server::conn::AddrStream| {
        let service = service.clone();
        async { Ok::<_, std::convert::Infallible>(service) }
    });

    let server = hyper::Server::bind(&env.addr).serve(make_service);
    println!("\n{}", env);
    let finisher = server.with_graceful_shutdown(shutdown());
    match finisher.await {
      Err(e) => eprintln!("Server error: {}", e),
       _ => println!("\nServer turn off"),
    }
}
