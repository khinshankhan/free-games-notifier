pub trait HttpClient {
    fn fetch_offers(&self) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct Client;

impl HttpClient for Client {
    fn fetch_offers(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://store-site-backend-static.ak.epicgames.com/freeGamesPromotions?locale=en-US&country=US&allowCountries=US";

        Ok(reqwest::blocking::get(url)
            .map_err(|e| {
                tracing::error!("Failed to fetch offers from Epic Games Store: {}", e);
                e
            })?
            .text()
            .map_err(|e| {
                tracing::error!("Failed to read response text from Epic Games Store: {}", e);
                e
            })?)
    }
}

pub struct MockClient {
    response_body: String,
}

impl MockClient {
    pub fn new(response_body: String) -> Self {
        Self { response_body }
    }
}

impl HttpClient for MockClient {
    fn fetch_offers(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.response_body.clone())
    }
}
