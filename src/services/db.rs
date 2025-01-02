use rusqlite::{params, Connection};
use crate::models::onion::Onion;

pub fn get_onions(conn: &Connection) -> Vec<Onion> {
    let mut stmt = conn
        .prepare("SELECT id, address, status, status_nb FROM onions")
        .expect("Failed to prepare statement");

    let onion_iter = stmt
        .query_map(params![], |row| {
            Ok(Onion {
                id: row.get(0)?,
                address: row.get(1)?,
                status: row.get(2)?,
                status_nb: row.get(3)?,
            })
        })
        .expect("Failed to query database");

    onion_iter.map(|o| o.unwrap()).collect()
}

pub fn update_onion_status(conn: &Connection, id: i32, status: &str) {
    conn.execute(
        "UPDATE onions SET status = ? WHERE id = ?",
        params![status, id],
    )
    .expect("Failed to update database");
}
