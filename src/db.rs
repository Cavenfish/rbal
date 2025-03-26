use std::fs;
use std::path::Path;

use dirs::data_dir;
use rusqlite::Connection;

pub fn create_new_db() {
  let mut db_file = data_dir().unwrap();
  db_file.set_file_name("rbal/main.db");

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
  let mut db_file = data_dir().unwrap();
  db_file.set_file_name("rbal/main.db");

  let db = Connection::open(db_file)
    .expect("Failed to open database");

  db
}

pub fn init_local() {
  let mut share = data_dir().unwrap();
  share.set_file_name("rbal/");
  
  if !Path::new(&share).exists()  {
    fs::create_dir_all(&share).expect("fail");
  }

  share.set_file_name("rbal/main.db");

  if !Path::new(&share).exists()  {
      create_new_db();
  }

}