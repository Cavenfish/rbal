use std::env;
use dotenv::dotenv;

use rusqlite::{params, Connection, Result};

pub fn create_new_db() {
  dotenv().ok();

  let db = Connection::open(
    env::var("DB_FILE").expect("DB_FILE must be set")
  ).unwrap();

  db.execute(
    "CREATE TABLE transactions (
        id      INTEGER PRIMARY KEY,
        type    TEXT,
        where   TEXT,
        message TEXT,
        coin    TEXT,
        netwrok TEXT,
        amount  REAL,
        date    TEXT
    )", ()
  ); 
}

pub fn load_db() -> Connection {
  dotenv().ok();

  let db = Connection::open(
    env::var("DB_FILE").expect("DB_FILE must be set")
  ).unwrap();

  db
}