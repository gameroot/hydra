use std::result;
use std::io::{Error, ErrorKind};

pub mod rocks_db;
pub mod json_db;
pub mod tests;


pub fn refresh(){
  rocks_db::refresh();
}

// return void?
pub fn load_from_json(){
  assert!(json_db::get_hosts().is_ok());
}

// return std::result w/Vec<String> payload
pub fn get_hosts() -> Result<(Vec<String>), Error> {
    json_db::get_hosts()
}
