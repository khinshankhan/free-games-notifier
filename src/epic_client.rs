pub trait EpicClient {
    fn fetch_offers(&self) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct RealClient;

impl EpicClient for RealClient {
    fn fetch_offers(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://store-site-backend-static.ak.epicgames.com/freeGamesPromotions?locale=en-US&country=US&allowCountries=US";

        Ok(reqwest::blocking::get(url)?.text()?)
    }
}
