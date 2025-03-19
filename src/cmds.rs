use crate::utils::{write_transaction, write_transactions,
                   get_transactions};
use crate::args::{TransInfo, ExportArgs};


pub fn add_trans(cmd: TransInfo) {

  write_transaction(&cmd, "./data/tmp.toml");
}

pub fn export_file(cmd: ExportArgs) {
  let db = get_transactions("./data/tmp.toml");

  write_transactions(&db, &cmd.filename);
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