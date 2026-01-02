use crate::offer_store::{ExistingOffer, OfferStore};

pub struct SqliteOfferStore {
    conn: rusqlite::Connection,
}

impl SqliteOfferStore {
    pub fn new(conn: rusqlite::Connection) -> Self {
        SqliteOfferStore { conn }
    }
}

impl OfferStore for SqliteOfferStore {
    fn ensure_schema(&self) -> rusqlite::Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS posted_offers (
              id        TEXT PRIMARY KEY,
              ends_at   INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_posted_offers_ends_at ON posted_offers(ends_at);
            "#,
        )?;
        Ok(())
    }

    fn get_existing_offers(&self) -> rusqlite::Result<Vec<ExistingOffer>> {
        let mut stmt = self.conn.prepare("SELECT id, ends_at FROM posted_offers")?;
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

    fn insert_offer(&self, offer_id: &str, ends_at: i64) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO posted_offers (id, ends_at) VALUES (?1, ?2)",
            rusqlite::params![offer_id, ends_at],
        )?;
        Ok(())
    }

    fn prune_expired_offers(&self, current_time: i64) -> rusqlite::Result<usize> {
        let affected = self.conn.execute(
            "DELETE FROM posted_offers WHERE ends_at < ?1",
            rusqlite::params![current_time],
        )?;
        Ok(affected)
    }
}
