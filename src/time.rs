pub trait TimeSource {
    fn now(&self) -> chrono::DateTime<chrono::Utc>;
}

pub struct SystemTimeSource;

impl TimeSource for SystemTimeSource {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }
}
