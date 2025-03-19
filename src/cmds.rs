use crate::utils::{write_transaction, 
                   get_transactions};
use crate::args::TransInfo;


pub fn add_trans(cmd: TransInfo) {

  write_transaction(&cmd, "./data/tmp.toml");
}

pub fn balance() {
  let trans = get_transactions("./data/tmp.toml");

  let mut u_tot: f64 = 0.0;
  let mut c_tot: f64 = 0.0;
  let mut total: f64 = 0.0;

  for i in trans.trans {

    if i.amount < 0.0 {
      u_tot += -i.amount;
    } else {
      c_tot += i.amount;
    }

    total += i.amount;
  };

  println!("USD   spent : {:.2} USD", &u_tot);
  println!("Crpto spent : {:.2} USD", &c_tot);
  println!("-------------------------------");
  println!("Net   spent : {:.2} USD", &total);
}

pub fn show() {
  let trans = get_transactions("./data/tmp.toml");

  println!("Type    Where    Coin    Amount");

  for i in trans.trans {
    println!("{}    {}    {}    {}", &i.t, &i.w, &i.coin, &i.amount);
  };

}