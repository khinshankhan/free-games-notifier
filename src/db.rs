use rusqlite::{Connection, params};

#[derive(Debug)]
pub struct ExistingOffer {
    pub id: String,
    pub ends_at: i64,
}

pub fn init_db(path: &str) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS posted_offers (
          id        TEXT PRIMARY KEY,
          ends_at   INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_posted_offers_ends_at ON posted_offers(ends_at);
        "#,
    )?;
    Ok(conn)
}

pub fn get_existing_offers(conn: &Connection) -> rusqlite::Result<Vec<ExistingOffer>> {
    let mut stmt = conn.prepare("SELECT id, ends_at FROM posted_offers")?;
    let mut rows = stmt.query([])?;

    let mut offers = Vec::new();
    while let Some(row) = rows.next()? {
        offers.push(ExistingOffer {
            id: row.get(0)?,
            ends_at: row.get(1)?,
        });
    }

    Ok(offers)
}

pub fn insert_offer(conn: &Connection, offer_id: &str, ends_at: i64) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO posted_offers (id, ends_at) VALUES (?1, ?2)",
        params![offer_id, ends_at],
    )?;
    Ok(())
}

pub fn prune_expired_offers(conn: &Connection, current_time: i64) -> rusqlite::Result<usize> {
    let affected = conn.execute(
        "DELETE FROM posted_offers WHERE ends_at < ?1",
        params![current_time],
    )?;
    Ok(affected)
}
