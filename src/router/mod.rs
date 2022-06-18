mod routes;
use std::convert::Infallible;
use hyper::{Request, Body, Response, Method};


/// Формирует ответ из кода HTTP.
pub fn from_code_and_msg(code: u16, msg: Option<&str>) -> Response<Body> {
  Response::builder()
    .header("Content-Type", "text/html; charset=utf-8")
    // .header("Access-Control-Allow-Origin", "http://localhost:3000")
    // .header("Access-Control-Allow-Credentials", "true")
    .status(code)
    .body(match msg {
      None => Body::empty(),
      Some(msg) => Body::from(String::from(msg)),
    })
    .unwrap()
}

pub async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
  Ok(match (req.method(), req.uri().path()) {
    (&Method::GET, "/") => routes::main_route(),
    _ => from_code_and_msg(404, Some("404: Route not found"))
  })
}