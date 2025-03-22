use std::fs;
// use std::io::Write;
use std::path::Path;
use toml::{from_str};

use std::env;
use dotenv::dotenv;
use rusqlite::{Connection, Statement};

use crate::args::TransInfo;
use crate::db::create_new_db;

// External
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TransVec {

  pub trans: Vec<TransInfo>,

}

pub fn get_rows(db: &Connection, query: &str) -> Vec<TransInfo> {

  let mut stmt = db.prepare(query).unwrap();

  let tmp = stmt.query_map([], |row| {
    Ok(TransInfo {

      vendor: row.get(1)?,

      message: row.get(2)?,

      coin: row.get(3)?,

      network: row.get(4)?,

      amount: row.get(5)?,

      date: row.get(6)?,
    })
  }).unwrap();

  let mut rows = Vec::new();

  for i in tmp {
    rows.push(i.unwrap())
  };

  rows
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