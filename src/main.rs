use chrono::{DateTime, Utc};
mod db;
mod discord;
mod epic;

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

    let conn = db::init_db("offers.db")?;
    db::prune_expired_offers(&conn, now.timestamp())?;

    let body = get_epic_data()?;
    let root = serde_json::from_str::<epic::Response>(&body)?;

    let existing_offers = db::get_existing_offers(&conn)?;
    let existing_offer_ids: std::collections::HashMap<String, i64> = existing_offers
        .into_iter()
        .map(|offer| (offer.id, offer.ends_at))
        .collect();

    let offers = root.data.catalog.search_store.elements;
    for offer in offers {
        let ends_at = match free_promo_ends_at(&offer, now) {
            Some(t) => t,
            None => continue,
        };

        if existing_offer_ids.contains_key(&offer.id) {
            println!("Offer '{}' already posted, skipping.", offer.title);
            continue;
        }

        let is_bundle = offer
            .offer_type
            .is_some_and(|ot| ot == epic::OfferType::Bundle)
            || offer.categories.is_some_and(|cats| {
                cats.iter()
                    .any(|ct| ct.path == "bundles" || ct.path == "bundles/games")
            });

        let ends_unix = ends_at.timestamp();
        let ends_rel = format!("<t:{ends_unix}:R>");

        let store_link = match offer.product_slug {
            Some(slug) if is_bundle => format!("https://store.epicgames.com/en-US/bundles/{slug}"),
            Some(slug) => format!("https://www.epicgames.com/store/en-US/p/{slug}"),
            None => continue,
        };

        let message = format!(
            "**{}** is now free on Epic Games Store! Ends {}\n{}",
            offer.title, ends_rel, store_link
        );

        if ALLOW_POST_FLAG {
            discord::send_webhook(&message)?;
            db::insert_offer(&conn, &offer.id, ends_unix)?;
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
