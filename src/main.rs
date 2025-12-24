mod epic;

use epic::{EpicResponse};
use chrono::{DateTime, Utc};

fn get_epic_data() -> Result<String, reqwest::Error> {
    let epic_url =
        "https://store-site-backend-static.ak.epicgames.com/freeGamesPromotions?locale=en-US";

    Ok(reqwest::blocking::get(epic_url)?.text()?)
}

fn is_free_now(offer: &epic::Offer, now: DateTime<Utc>) -> bool {
    if offer.price.total_price.discount_price != 0 {
        return false;
    }

    let Some(promotions) = &offer.promotions else {
        return false;
    };

    for block in &promotions.promotional_offers {
        for promo in &block.promotional_offers {
            if promo.discount_setting.discount_percentage != Some(0) {
                continue;
            }

            let start = DateTime::parse_from_rfc3339(&promo.start_date)
                .ok()
                .map(|dt| dt.with_timezone(&Utc));
            let end = DateTime::parse_from_rfc3339(&promo.end_date)
                .ok()
                .map(|dt| dt.with_timezone(&Utc));

            if let (Some(start), Some(end)) = (start, end) {
                if start <= now && now < end {
                    return true;
                }
            }
        }
    }

    false
}

fn handle_epic() -> Result<(), Box<dyn std::error::Error>> {
    let now = chrono::Utc::now();

    let body = get_epic_data()?;
    let root = serde_json::from_str::<EpicResponse>(&body)?;

    let offers = root.data.catalog.search_store.elements;
    for offer in offers.iter().filter(|o| is_free_now(o, now)) {
        println!("FREE NOW: {} ({}:{})", offer.title, offer.namespace, offer.id);
    }

    Ok(())
}

fn main() {
    match handle_epic() {
        Ok(()) => println!("Successfully fetched and displayed Epic Games offers."),
        Err(e) => eprintln!("HTTP error: {e}"),
    }
}
