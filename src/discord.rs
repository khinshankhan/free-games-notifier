use std::env;

pub fn send_discord_webhook(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let webhook_url = env::var("DISCORD_WEBHOOK_URL")
        .map_err(|_| "DISCORD_WEBHOOK_URL not set")?;

    let payload = serde_json::json!({
        "content": message
    });

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(webhook_url)
        .json(&payload)
        .send()?;

    if !resp.status().is_success() {
        return Err(format!("Discord webhook failed: {}", resp.status()).into());
    }

    Ok(())
}
