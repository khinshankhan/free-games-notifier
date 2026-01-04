pub trait Notifier {
    fn notify(&self, message: &str) -> Result<(), Box<dyn std::error::Error>>;
}

mod capture;
mod discord;
mod logging;

pub use capture::*;
pub use discord::*;
pub use logging::*;
