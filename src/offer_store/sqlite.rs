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
        self.conn
            .execute_batch(
                r#"
            CREATE TABLE IF NOT EXISTS posted_offers (
              id        TEXT PRIMARY KEY,
              source    TEXT NOT NULL,
              ends_at   INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_posted_offers_ends_at ON posted_offers(ends_at);
            "#,
            )
            .map_err(|e| {
                tracing::error!("Failed to ensure schema: {e}");
                e
            })?;

        Ok(())
    }

    fn get_existing_offers(&self) -> rusqlite::Result<Vec<ExistingOffer>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, source, ends_at FROM posted_offers")?;
        let mut rows = stmt.query([]).map_err(|e| {
            tracing::error!("Failed to query existing offers: {e}");
            e
        })?;

        let mut offers = Vec::new();
        while let Some(row) = rows.next()? {
            offers.push(ExistingOffer {
                id: row.get(0)?,
                source: row.get(1)?,
                ends_at: row.get(2)?,
            });
        }

        Ok(offers)
    }

    fn insert_offer(&self, id: &str, source: &str, ends_at: i64) -> rusqlite::Result<()> {
        self.conn
            .execute(
                "INSERT INTO posted_offers (id, source, ends_at) VALUES (?1, ?2, ?3)",
                rusqlite::params![id, source, ends_at],
            )
            .map_err(|e| {
                tracing::error!("Failed to insert offer {}: {e}", id);
                e
            })?;

        Ok(())
    }

    fn prune_expired_offers(&self, current_time: i64) -> rusqlite::Result<usize> {
        let affected = self
            .conn
            .execute(
                "DELETE FROM posted_offers WHERE ends_at < ?1",
                rusqlite::params![current_time],
            )
            .map_err(|e| {
                tracing::error!("Failed to prune expired offers: {e}");
                e
            })?;

        Ok(affected)
    }
}
