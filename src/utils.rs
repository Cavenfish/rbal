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
        println!(
            "{: <5} {: <15} {: <15} {: <10} {: <5}",
            &row.id, &row.vendor, &row.date, &row.coin, &row.amount
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
