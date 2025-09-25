use crate::args::{ExportArgs, ImportArgs, RemoveArgs, ShowArgs, TransInfo};
use crate::db::load_db;
use crate::utils::{get_rows, show_all, show_id};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

use chrono::Local;
use rusqlite::Connection;
use rusqlite::vtab::csvtab::load_module;

pub fn export_file(cmd: ExportArgs) {
    let rows: Vec<TransInfo> = get_rows();

    let mut file = File::create(&cmd.filename).expect("Unable to create file");

    writeln!(file, "id,vendor,message,coin,network,amount,date").expect("Fail to write");

    for row in rows {
        writeln!(
            file,
            "{},{},{},{},{},{},{}",
            &row.id, &row.vendor, &row.message, &row.coin, &row.network, &row.amount, &row.date
        )
        .expect("Fail to write");
    }
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

    let vtab = format!(
        "
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
    FROM csv_data",
        (),
    )
    .expect("fail");

    db.execute("DROP TABLE csv_data", ()).expect("fail");
}

pub fn add_trans(cmd: TransInfo) {
    let db: Connection = load_db();

    let date = match cmd.date.as_str() {
        // Handle default value
        "today" => {
            let tmp = Local::now();

            &tmp.format("%Y-%m-%d").to_string()
        }

        // Handle user input
        _ => &cmd.date,
    };

    db.execute(
        "INSERT INTO rbal (
    vendor, message, coin, network, amount, date)
    values (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            &cmd.vendor,
            &cmd.message,
            &cmd.coin,
            &cmd.network,
            &cmd.amount,
            date,
        ),
    )
    .expect("Failed to add transaction");
}

pub fn remove_trans(cmd: RemoveArgs) {
    let db: Connection = load_db();

    db.execute("DELETE FROM rbal WHERE id = ?1", (cmd.id,))
        .expect("Failed to remove transaction");
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

pub fn coins() {
    let db: Connection = load_db();

    let mut coins_map = BTreeMap::new();

    let mut stmt = db.prepare("SELECT coin, amount FROM rbal").unwrap();
    let mut rows = stmt.query([]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        let k: String = row.get(0).unwrap();
        let v: f64 = row.get(1).unwrap();

        if k == "N/A" {
            continue;
        }

        if coins_map.contains_key(&k) {
            let (v0, i0) = coins_map.get(&k).unwrap();
            coins_map.insert(k, (v0 + v, i0 + 1));
        } else {
            coins_map.insert(k, (v, 1));
        }
    }

    println!("{: <8} {: <8} {: <5}", "Coin", "Total", "Tx Count");

    println!("{:-<30}", "");

    for (coin, (total, txs)) in &coins_map {
        println!("{: <6} {: >7.2} {: >7}", coin, total, txs);
    }
}

pub fn show(cmd: ShowArgs) {
    match cmd.id {
        // Handle default value
        0 => show_all(),

        // Handle user input
        _ => show_id(cmd.id),
    };
}
