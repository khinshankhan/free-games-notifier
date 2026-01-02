pub trait TimeSource {
    fn now(&self) -> chrono::DateTime<chrono::Utc>;
}

pub struct SystemTimeSource;

impl TimeSource for SystemTimeSource {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }
}

pub fn parse_utc(s: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.to_utc())
}
