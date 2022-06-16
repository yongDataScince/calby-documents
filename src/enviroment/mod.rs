use std::{env, net::SocketAddr};
use std::fmt::Display;

#[derive(Debug)]
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

fn not_provided(var: &str) -> String {
  format!("variable {} not provided", var)
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
}

#[derive(Debug)]
pub struct Enviroment {
  pub host: String,
  pub port: String,
  pub format_addr: String,
  pub addr: SocketAddr,
  pub mode: Mode
}

impl Display for Enviroment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Server started on address: {}\ndevelopment mode: {}", self.format_addr, self.mode)
    }
}
