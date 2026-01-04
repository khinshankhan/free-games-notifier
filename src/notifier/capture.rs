use crate::notifier::{Notifier};

pub struct CaptureNotifier {
    msgs: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
}

impl CaptureNotifier {
    pub fn new() -> Self {
        CaptureNotifier {
            msgs: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub fn get_messages(&self) -> Vec<String> {
        self.msgs.lock().unwrap().clone()
    }
}

impl Notifier for CaptureNotifier {
    fn notify(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.msgs.lock().unwrap().push(message.to_string());
        Ok(())
    }
}
