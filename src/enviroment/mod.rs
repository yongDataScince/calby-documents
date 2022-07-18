use std::{env, net::SocketAddr};
use std::fmt::Display;
use crate::service::DocumentsServise;
use crate::psql_handler::Db;

pub fn not_provided(var: &str) -> String {
  format!("variable {} not provided", var)
}

#[derive(Debug, Clone)]
pub enum Mode {
  Development,
  Production
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Mode::*;
        match self {
          Production => writeln!(f, "Production"),
          Development => writeln!(f, "Development")
        }
    }
}

impl Mode {
    pub fn get_host_port(&self) -> (String, String) {
      use Mode::*;
      let host: String;
      let port: String;

      match self {
        Development => {
          host = env::var("DEV_HOST").expect(&not_provided("DEV_HOST"));
          port = env::var("DEV_PORT").expect(&not_provided("DEV_PORT"));
        },
        Production => {
          host = env::var("PROD_HOST").expect(&not_provided("PROD_HOST"));
          port = env::var("PROD_PORT").expect(&not_provided("PROD_PORT"));
        }
      };
      (host, port)
    }

    pub fn get_mode() -> Self {
      match env::var("MODE") {
        Ok(value) if value == String::from("production") => Self::Production,
        Ok(value) if value == String::from("development") => Self::Development,
        Err(_) => panic!("{}", &not_provided("MODE")),
        Ok(value) => panic!("Unexpected value: {}", value)
      }
    }
}

#[derive(Debug, Clone)]
pub struct Enviroment {
  pub host: String,
  pub port: String,
  pub format_addr: String,
  pub addr: SocketAddr,
  pub mode: Mode,
  pub service: DocumentsServise
}

impl Display for Enviroment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Server started on address: {}\ndevelopment mode: {}", self.format_addr, self.mode)
    }
}

impl Enviroment {
  pub fn init(db: Db) -> Self {
    let mode = Mode::get_mode();
    let (host, port) = mode.get_host_port();
    let format_addr = format!("http://{}:{}", &host, &port);
    let addr: SocketAddr = format!("{}:{}", &host, &port).parse().unwrap();
    let docs_service = DocumentsServise { db };

    Enviroment {
      service: docs_service,
      mode,
      host,
      port,
      format_addr,
      addr
    }
  }
}
