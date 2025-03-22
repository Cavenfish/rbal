use std::env;
use dotenv::dotenv;

use rusqlite::Connection;

pub fn create_new_db() {
  dotenv().ok();

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
  dotenv().ok();

  let db = Connection::open(
    env::var("DB_FILE").expect("DB_FILE must be set")
  ).expect("Failed to open database");

  db
}