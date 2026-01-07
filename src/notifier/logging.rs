use crate::notifier::Notifier;

pub struct LoggingNotifier;

impl Notifier for LoggingNotifier {
    fn notify(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Notification: {}", message);
        Ok(())
    }
}
