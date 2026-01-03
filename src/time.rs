pub trait TimeSource {
    fn now(&self) -> chrono::DateTime<chrono::Utc>;
}

pub struct SystemTimeSource;

impl TimeSource for SystemTimeSource {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }
}

pub struct MockTimeSource {
    fixed_time: chrono::DateTime<chrono::Utc>,
}

impl MockTimeSource {
    pub fn new(fixed_time: chrono::DateTime<chrono::Utc>) -> Self {
        Self { fixed_time }
    }
}

impl TimeSource for MockTimeSource {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        self.fixed_time
    }
}

pub fn parse_utc(s: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map_err(|e| {
            tracing::error!("Failed to parse datetime '{}': {}", s, e);
            e
        })
        .ok()
        .map(|dt| dt.to_utc())
}
