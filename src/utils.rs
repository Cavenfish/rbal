use std::fs;
use std::io::Write;
use std::path::Path;
use toml::{from_str, to_string};

use std::env;
use dotenv::dotenv;

use crate::args::TransInfo;
use crate::db::create_new_db;

// External
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TransVec {

  pub trans: Vec<TransInfo>,

}

pub fn get_transactions(file: &str) -> TransVec {
  let f = fs::read_to_string(file).expect("Failed");

  let trans: TransVec = from_str(&f).unwrap();

  trans
}

pub fn init_local() {
  dotenv().ok();

  let p: String = env::var("RBAL_DIR")
    .expect("RBAL_DIR must be set");
  
  if !Path::new(&p).exists()  {
    fs::create_dir_all(&p).expect("fail");
  }

  let f:String = env::var("DB_FILE")
    .expect("DB_FILE must be set");

  if !Path::new(&f).exists()  {
      create_new_db();
  }

}