use std::fmt::Write as _;
use std::fs::File;
use std::io::Write as _;

use anyhow::{Context, Result};
use rusqlite::vtab::csvtab::load_module;
use rusqlite::{Connection, ToSql};

use crate::args::{EditArgs, ExportArgs, ImportArgs, RemoveArgs, ShowArgs, TransInfo};
use crate::utils::{get_coins_data, get_rows, handle_date, show_all, show_id};

pub fn export_file(cmd: ExportArgs, db: Connection) {
    let rows: Vec<TransInfo> = get_rows(db);

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

pub fn import_file(cmd: ImportArgs, db: Connection) -> Result<()> {
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

    db.execute_batch(&vtab)
        .context("Failed to create virtual table")?;

    db.execute(
        "INSERT INTO rbal SELECT 
      CAST(id AS INTEGER), 
      vendor, message, coin, network,
      CAST(amount AS REAL),
      date
    FROM csv_data",
        (),
    )
    .context("Failed to insert data into rbal table")?;

    db.execute("DROP TABLE csv_data", ())
        .context("Failed to drop virtual table")?;

    Ok(())
}

pub fn add_trans(cmd: TransInfo, db: Connection) {
    let date = handle_date(&cmd.date).expect("Failed to interpret date");

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

pub fn edit_trans(cmd: EditArgs, db: Connection) {
    let mut i = 1;
    let mut params: Vec<&dyn ToSql> = Vec::new();
    let mut sql = String::with_capacity(1024);

    sql.push_str("UPDATE rbal SET ");

    if let Some(ref vendor) = cmd.vendor {
        write!(&mut sql, "vendor = ?{}, ", i).unwrap();
        params.push(vendor);
        i += 1;
    }

    if let Some(ref message) = cmd.message {
        write!(&mut sql, "message = ?{}, ", i).unwrap();
        params.push(message);
        i += 1;
    }

    if let Some(ref coin) = cmd.coin {
        write!(&mut sql, "coin = ?{}, ", i).unwrap();
        params.push(coin);
        i += 1;
    }

    if let Some(ref network) = cmd.network {
        write!(&mut sql, "network = ?{}, ", i).unwrap();
        params.push(network);
        i += 1;
    }

    if let Some(ref amount) = cmd.amount {
        write!(&mut sql, "amount = ?{}, ", i).unwrap();
        params.push(amount);
        i += 1;
    }

    let new_date: String;
    if let Some(ref date) = cmd.date {
        new_date = handle_date(date).expect("Failed to interpret date");

        write!(&mut sql, "date = ?{}, ", i).unwrap();
        params.push(&new_date);
        i += 1;
    }

    if params.is_empty() {
        return;
    }

    sql.truncate(sql.len() - 2);

    write!(&mut sql, " WHERE id = ?{}", i).unwrap();
    params.push(&cmd.id);

    db.execute(&sql, &params[..])
        .expect("Failed to edit transaction");
}

pub fn remove_trans(cmd: RemoveArgs, db: Connection) {
    db.execute("DELETE FROM rbal WHERE id = ?1", (cmd.id,))
        .expect("Failed to remove transaction");
}

pub fn balance(db: Connection) {
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

pub fn coins(db: Connection) {
    let coins_map = get_coins_data(db).unwrap();

    println!("{: <8} {: <8} {: <5}", "Coin", "Total", "Tx Count");

    println!("{:-<30}", "");

    for (coin, (total, txs)) in &coins_map {
        println!("{: <6} {: >7.2} {: >7}", coin, total, txs);
    }
}

pub fn show(cmd: ShowArgs, db: Connection) {
    match cmd.id {
        // Handle default value
        0 => show_all(db),

        // Handle user input
        _ => show_id(cmd.id, db),
    };
}
