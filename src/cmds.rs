use crate::db::load_db;
use crate::args::{TransInfo, ExportArgs};
use crate::utils::get_transactions;

use std::env;
use dotenv::dotenv;

use rusqlite::Connection;

pub fn add_trans(cmd: TransInfo) {
  dotenv().ok();

  let db = Connection::open(
    env::var("DB_FILE").expect("DB_FILE must be set")
  ).unwrap();
  
  db.execute(
    "INSERT INTO transactions (type, 
    where, message, coin, network, amount, date)
    values ()", 
    (&cmd.t, &cmd.w, &cmd.f, &cmd.coin,
     &cmd.network, &cmd.amount, &cmd.date)
  );

}

pub fn export_file(cmd: ExportArgs) {
  dotenv().ok();

  let db = Connection::open(
    env::var("DB_FILE").expect("DB_FILE must be set")
  );

  // write_transactions(&db, &cmd.filename);
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
  let db = get_transactions("./data/tmp.toml");

  println!(
    "{: <15} {: <15} {: <10} {}",
    "Type", "Where", "Coin", "Amount"
  );

  println!("{:-<55}", "");

  for i in db.trans {
    println!(
      "{: <15} {: <15} {: <10} {}", 
      &i.t, &i.w, &i.coin, &i.amount
    );
  };

}