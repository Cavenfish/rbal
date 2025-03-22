use crate::db::load_db;
use crate::args::{TransInfo, ExportArgs, ImportArgs, ShowArgs};
use crate::utils::{TransVec, get_transactions, get_rows};

use rusqlite::Connection;


pub fn export_file(cmd: ExportArgs) {
  let db: Connection = load_db();

  // db.execute(
  //   ".mode csv;
  //   "
  // )
}

pub fn import_file(cmd: ImportArgs) {
  let tr: TransVec   = get_transactions(&cmd.filename);

  for i in tr.trans {
    add_trans(i);
  };

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

pub fn show(cmd: ShowArgs) {
  let db: Connection = load_db();

  println!(
    "{: <15} {: <15} {: <10} {}",
    "Vendor", "Date", "Coin", "Amount"
  );

  println!("{:-<55}", "");

  let query: String = format!(
    "SELECT {} FROM rbal", &cmd.query
  );

  let rows: Vec<TransInfo> = get_rows(&db, &query);

  for row in rows {
    println!(
      "{: <15} {: <15} {: <10} {}", 
      &row.vendor, &row.date, &row.coin, &row.amount
    );
  };

}