use std::collections::BTreeMap;

use anyhow::Result;
use colored::Colorize;
use rusqlite::Connection;

use crate::args::TransInfo;
use crate::db::load_db;

pub fn get_rows() -> Vec<TransInfo> {
    let db: Connection = load_db();

    let mut stmt = db.prepare("SELECT * FROM rbal").unwrap();

    let tmp = stmt
        .query_map([], |row| {
            Ok(TransInfo {
                id: row.get(0)?,

                vendor: row.get(1)?,

                message: row.get(2)?,

                coin: row.get(3)?,

                network: row.get(4)?,

                amount: row.get(5)?,

                date: row.get(6)?,
            })
        })
        .unwrap();

    let trans = tmp.collect::<Result<Vec<TransInfo>, _>>();

    trans.expect("fail")
}

pub fn show_all() {
    println!(
        "{: <5} {: <15} {: <15} {: <10} {: <5}",
        "ID", "Vendor", "Date", "Coin", "Amount"
    );

    println!("{:-<60}", "");

    let rows: Vec<TransInfo> = get_rows();

    for row in rows {
        let amt = if row.amount < 0.0 {
            row.amount.to_string().red()
        } else {
            row.amount.to_string().green()
        };

        println!(
            "{: <5} {: <15} {: <15} {: <10} {: <5}",
            &row.id, &row.vendor, &row.date, &row.coin, &amt
        );
    }
}

pub fn show_id(id: u32) {
    let db: Connection = load_db();

    let query = format!("SELECT * FROM rbal WHERE id = {}", id,);

    let row: TransInfo = db
        .query_row(&query, [], |row| {
            Ok(TransInfo {
                id: row.get(0)?,

                vendor: row.get(1)?,

                message: row.get(2)?,

                coin: row.get(3)?,

                network: row.get(4)?,

                amount: row.get(5)?,

                date: row.get(6)?,
            })
        })
        .unwrap();

    println!("{}", row);
}

pub fn get_coins_data() -> Result<BTreeMap<String, (f64, i32)>> {
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

    Ok(coins_map)
}
