use std::fs;
use std::env;
use std::path::Path;

use rusqlite::Connection;

pub fn create_new_db() {

  let db = Connection::open(
    env::var("DB_FILE").expect("DB_FILE must be set")
  ).expect("Failed to open database");

  db.execute(
    "CREATE TABLE rbal (
        id      INTEGER PRIMARY KEY,
        vendor  TEXT,
        message TEXT,
        coin    TEXT,
        network TEXT,
        amount  REAL,
        date    TEXT
    )", ()
  ).expect("Failed to make table"); 
}

pub fn load_db() -> Connection {

  let db = Connection::open(
    env::var("DB_FILE").expect("DB_FILE must be set")
  ).expect("Failed to open database");

  db
}

pub fn init_local() {

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