mod sqlite;

#[derive(Debug)]
pub struct ExistingOffer {
    pub id: String,
    pub ends_at: i64,
}

pub trait OfferStore {
    fn ensure_schema(&self) -> rusqlite::Result<()>;
    fn get_existing_offers(&self) -> rusqlite::Result<Vec<ExistingOffer>>;
    fn insert_offer(&self, offer_id: &str, ends_at: i64) -> rusqlite::Result<()>;
    fn prune_expired_offers(&self, current_time: i64) -> rusqlite::Result<usize>;
}

pub use sqlite::SqliteOfferStore;
