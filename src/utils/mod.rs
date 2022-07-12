use std::env;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn hash_string(input_str: String) -> String {
  let mut hasher = Sha256::new();
  let salt = env::var("SALT").unwrap();
  hasher.input(&[input_str.as_bytes(), salt.as_bytes()].concat());
  hasher.result_str()
}