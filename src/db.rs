use std::env;
use dotenv::dotenv;

use rusqlite::Connection;

pub fn create_new_db() {
  dotenv().ok();

  let db = Connection::open(
    env::var("DB_FILE").expect("DB_FILE must be set")
  ).unwrap();

  db.execute(
    "CREATE TABLE transactions (
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
  dotenv().ok();

  let db = Connection::open(
    env::var("DB_FILE").expect("DB_FILE must be set")
  ).unwrap();

  db
}