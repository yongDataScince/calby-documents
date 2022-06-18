use super::*;

pub fn main_route() -> Response<Body> {
  from_code_and_msg(201, Some("main route"))
}


