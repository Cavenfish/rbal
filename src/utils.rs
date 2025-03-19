use std::fs;
use std::io::Write;
use toml::{from_str, to_string};

use crate::args::TransInfo;

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

pub fn write_transaction(info: &TransInfo, file: &str) {

  let mut f = fs::OpenOptions::new()
      .append(true)
      .create(true)
      .open(file)
      .unwrap();

  let s: String = to_string(info).expect("fail");
  
  writeln!(f, "[[trans]]").expect("fail");
  writeln!(f, "{}", &s).expect("fail");
}

pub fn write_transactions(trans: &TransVec, file: &str) {

  let s: String = to_string(trans).expect("fail");

  fs::write(file, &s).expect("fail");

}