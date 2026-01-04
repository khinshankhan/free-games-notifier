use crate::notifier::{Notifier};

pub struct DiscordNotifier {
    webhook_url: String,
    client: reqwest::blocking::Client,
}

impl DiscordNotifier {
    pub fn new(webhook_url: String) -> Self {
        DiscordNotifier {
            webhook_url: webhook_url,
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl Notifier for DiscordNotifier {
    fn notify(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::json!({
            "content": message
        });

        let resp = self
            .client
            .post(self.webhook_url.clone())
            .json(&payload)
            .send()
            .map_err(|e| format!("Failed to send Discord webhook: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Discord webhook failed: {}", resp.status()).into());
        }

        Ok(())
    }
}
