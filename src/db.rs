use std::fs;
use std::path::Path;

use dirs::data_dir;
use rusqlite::Connection;

pub fn create_new_db() {
  let db_file = data_dir().unwrap().join("rbal/main.db");

  let db = Connection::open(db_file)
    .expect("Failed to open database");

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
  let db_file = data_dir().unwrap().join("rbal/main.db");

  let db = Connection::open(db_file)
    .expect("Failed to open database");

  db
}

pub fn init_local() {
  let rbal = data_dir().unwrap().join("rbal");
  
  if !Path::new(&rbal).exists()  {
    fs::create_dir_all(&rbal).expect("fail");
  }

  let db_file = rbal.join("main.db");

  if !Path::new(&db_file).exists()  {
      create_new_db();
  }

}