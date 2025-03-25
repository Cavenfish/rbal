use std::fs;
use std::path::Path;

use std::env;
use dotenv::dotenv;
use rusqlite::{Connection};

use crate::args::TransInfo;
use crate::db::create_new_db;

pub fn get_rows(db: &Connection) -> Vec<TransInfo> {

  let mut stmt = db.prepare("SELECT * FROM rbal").unwrap();

  let tmp = stmt.query_map([], |row| {
    Ok(TransInfo {

      id: row.get(0)?,

      vendor: row.get(1)?,

      message: row.get(2)?,

      coin: row.get(3)?,

      network: row.get(4)?,

      amount: row.get(5)?,

      date: row.get(6)?,
    })
  }).unwrap();


  let trans = tmp.collect::<Result<Vec<TransInfo>, _>>();

  trans.expect("fail")
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