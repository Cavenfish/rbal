use crate::db::load_db;
use crate::args::{TransInfo, ExportArgs, ImportArgs};
use crate::utils::{TransVec, get_transactions};

use rusqlite::Connection;


pub fn export_file(cmd: ExportArgs) {
  let db: Connection = load_db();

  // write_transactions(&db, &cmd.filename);
}

pub fn import_file(cmd: ImportArgs) {
  let db: Connection = load_db();
  let tr: TransVec   = get_transactions(&cmd.filename);

  for i in tr.trans {
    add_trans(i);
  };

}

pub fn add_trans(cmd: TransInfo) {
  let db: Connection = load_db();
  
  db.execute(
    "INSERT INTO transactions (
    vendor, message, coin, network, amount, date)
    values (?1, ?2, ?3, ?4, ?5, ?6)", 
    (&cmd.vendor, &cmd.message, &cmd.coin,
     &cmd.network, &cmd.amount, &cmd.date)
  ).expect("Failed to add transaction");

}

pub fn balance() {
  let db = get_transactions("./data/tmp.toml");

  let mut u_tot: f64 = 0.0;
  let mut c_tot: f64 = 0.0;
  let mut total: f64 = 0.0;

  for i in db.trans {

    if i.amount < 0.0 {
      u_tot += -i.amount;
    } else {
      c_tot += i.amount;
    }

    total += i.amount;
  };

  println!("{: <12}: {:0>8.2} USD", "USD Spent", &u_tot);
  
  println!("{: <12}: {:0>8.2} USD", "Crypto Spent", &c_tot);

  println!("{:-<25}", "");

  println!("{: <12}: {:0>8.2} USD", "Net Spent", &total);
}

pub fn show() {
  let db: Connection = load_db();

  println!(
    "{: <15} {: <15} {: <10} {}",
    "Type", "Where", "Coin", "Amount"
  );

  println!("{:-<55}", "");

  let mut stmt = db.prepare(
    "SELECT * FROM transactions"
  ).unwrap();

  let trans = stmt.query_map([], |row| {
    Ok(TransInfo {

      vendor: row.get(1)?,

      message: row.get(2)?,

      coin: row.get(3)?,

      network: row.get(4)?,

      amount: row.get(5)?,

      date: row.get(6)?,
    })
  }).unwrap();

  for i in trans {

    println!("{:?}", i);
    // println!(
    //   "{: <15} {: <15} {: <10} {}", 
    //   &i.t, &i.w, &i.coin, &i.amount
    // );
  };

}