use crate::notifier;

pub struct DiscordNotifier {
    webhook_url: String,
}

impl DiscordNotifier {
    pub fn new(webhook_url: String) -> Self {
        DiscordNotifier { webhook_url }
    }
}

impl notifier::Notifier for DiscordNotifier {
    fn notify(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::json!({
            "content": message
        });

        let client = reqwest::blocking::Client::new();
        let resp = client.post(&self.webhook_url).json(&payload).send()?;

        if !resp.status().is_success() {
            return Err(format!("Discord webhook failed: {}", resp.status()).into());
        }

        Ok(())
    }
}
