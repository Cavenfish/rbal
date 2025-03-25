use crate::db::load_db;
use crate::args::{TransInfo, ExportArgs, ImportArgs};
use crate::utils::{get_rows};

use std::fs::File;
use std::io::Write;

use rusqlite::Connection;
use rusqlite::vtab::csvtab::load_module;


pub fn export_file(cmd: ExportArgs) {
  let db: Connection = load_db();

  let rows: Vec<TransInfo> = get_rows(&db);

  let mut file = File::create(&cmd.filename)
    .expect("Unable to create file");

  writeln!(file, 
    "id,vendor,message,coin,network,amount,date"
  ).expect("Fail to write");

  for row in rows {
    writeln!(file,
      "{},{},{},{},{},{},{}",
      &row.id, &row.vendor, &row.message, &row.coin,
      &row.network, &row.amount, &row.date
    ).expect("Fail to write");
  };

}

pub fn import_file(cmd: ImportArgs) {

  let db: Connection = load_db();

  load_module(&db).expect("unable to load");

  let schema = "CREATE TABLE x (
        id      INTEGER,
        vendor  TEXT,
        message TEXT,
        coin    TEXT,
        network TEXT,
        amount  REAL,
        date    TEXT
    )";

  let vtab = format!("
    CREATE VIRTUAL TABLE csv_data
    USING csv(filename = '{}', schema = '{}', header=YES)", 
    &cmd.filename, &schema,
  );

  db.execute_batch(&vtab).expect("fail");

  db.execute(
    "INSERT INTO rbal SELECT 
      CAST(id AS INTEGER), 
      vendor, message, coin, network,
      CAST(amount AS REAL),
      date
    FROM csv_data", ()
  ).expect("fail");

  db.execute("DROP TABLE csv_data", ()).expect("fail");

}

pub fn add_trans(cmd: TransInfo) {
  let db: Connection = load_db();
  
  db.execute(
    "INSERT INTO rbal (
    vendor, message, coin, network, amount, date)
    values (?1, ?2, ?3, ?4, ?5, ?6)", 
    (&cmd.vendor, &cmd.message, &cmd.coin,
     &cmd.network, &cmd.amount, &cmd.date)
  ).expect("Failed to add transaction");

}

pub fn balance() {
  let db: Connection = load_db();

  let mut u_tot: f64 = 0.0;
  let mut c_tot: f64 = 0.0;
  let mut total: f64 = 0.0;

  let mut stmt = db.prepare("SELECT amount FROM rbal").unwrap();
  let mut rows = stmt.query([]).unwrap();

  while let Some(row) = rows.next().unwrap() {
    let a: f64 = row.get(0).unwrap();

    if a < 0.0 {
      u_tot += -a;
    } else {
      c_tot += a;
    }
    
    total += a;

  }

  println!("{: <12}: {:0>8.2} USD", "USD Spent", &u_tot);
  
  println!("{: <12}: {:0>8.2} USD", "Crypto Spent", &c_tot);

  println!("{:-<25}", "");

  println!("{: <12}: {:0>8.2} USD", "Net Spent", &total);
}

pub fn show() {
  let db: Connection = load_db();

  println!(
    "{: <5} {: <15} {: <15} {: <10} {}",
    "ID", "Vendor", "Date", "Coin", "Amount"
  );

  println!("{:-<55}", "");

  let rows: Vec<TransInfo> = get_rows(&db);

  for row in rows {
    println!(
      "{: <5} {: <15} {: <15} {: <10} {}", &row.id,
      &row.vendor, &row.date, &row.coin, &row.amount
    );
  };

}