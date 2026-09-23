use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use dirs::data_dir;
use rusqlite::Connection;

pub fn create_new_db() -> Result<()> {
    let db_file = data_dir().unwrap().join("rbal/main.db");

    let db = Connection::open(db_file).context("Failed to open database")?;

    db.execute(
        "CREATE TABLE rbal (
        id      INTEGER PRIMARY KEY,
        vendor  TEXT,
        message TEXT,
        coin    TEXT,
        network TEXT,
        amount  REAL,
        date    TEXT
    )",
        (),
    )
    .context("Failed to make table")?;

    Ok(())
}

pub fn load_db() -> Result<Connection> {
    let db_file = data_dir().unwrap().join("rbal/main.db");

    Connection::open(db_file).context("Failed to open database")
}

pub fn init_local() -> Result<()> {
    let rbal = data_dir().unwrap().join("rbal");

    if !Path::new(&rbal).exists() {
        fs::create_dir_all(&rbal).context("Failed to create data dir")?;
    }

    let db_file = rbal.join("main.db");

    if !Path::new(&db_file).exists() {
        create_new_db()?;
    }

    Ok(())
}
