use chrono::{DateTime, Utc};

mod epic;
use epic::{EpicResponse};

mod discord;
use discord::{send_discord_webhook};

fn get_epic_data() -> Result<String, reqwest::Error> {
    let epic_url =
        "https://store-site-backend-static.ak.epicgames.com/freeGamesPromotions?locale=en-US";

    Ok(reqwest::blocking::get(epic_url)?.text()?)
}

fn free_promo_ends_at(offer: &epic::Offer, now: DateTime<Utc>) -> Option<DateTime<Utc>> {
    if offer.price.total_price.discount_price != 0 {
        return None;
    }

    let Some(promotions) = &offer.promotions else {
        return None;
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
                    return Some(end);
                }
            }
        }
    }

    None
}

const ALLOW_POST_FLAG: bool = true;

fn handle_epic() -> Result<(), Box<dyn std::error::Error>> {
    let now = chrono::Utc::now();

    let body = get_epic_data()?;
    let root = serde_json::from_str::<EpicResponse>(&body)?;

    let offers = root.data.catalog.search_store.elements;
    for offer in offers {
        let ends_at = match free_promo_ends_at(&offer, now) {
            Some(t) => t,
            None => continue,
        };

        let is_bundle = 'is_bundle: {
            if let Some(offer_type) = &offer.offer_type {
                if offer_type == "BUNDLE" {
                    break 'is_bundle true;
                }
            }

            let Some(categories) = &offer.categories else {
                break 'is_bundle false;
            };

            for category in categories {
                if category.path == "bundles" || category.path == "bundles/games" {
                    break 'is_bundle true;
                }
            }

            break 'is_bundle false;
        };


        let ends_unix = ends_at.timestamp();
        let ends_rel = format!("<t:{ends_unix}:R>");

        let store_link = match &offer.product_slug {
            Some(slug) => {
                if is_bundle {
                    format!("https://store.epicgames.com/en-US/bundles/{}", slug)
                } else {
                    format!("https://www.epicgames.com/store/en-US/p/{}", slug)
                }
            },
            None => continue,
        };

        let message = format!(
            "**{}** is now free on Epic Games Store! Ends {}\n{}",
            offer.title, ends_rel, store_link
        );

        if ALLOW_POST_FLAG {
            send_discord_webhook(&message)?;
        } else {
            println!("Posting disabled. Message:\n{}", message);
            continue;
        }
    }

    Ok(())
}

fn main() {
    dotenvy::dotenv().ok();

    match handle_epic() {
        Ok(()) => println!("Successfully fetched and displayed Epic Games offers."),
        Err(e) => eprintln!("HTTP error: {e}"),
    }
}
