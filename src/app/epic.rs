use crate::{epic, notifier, offer_store, time};

pub struct Offer {
    id: String,
    source: String,
    title: String,
    link: String,
    ends_at: chrono::DateTime<chrono::Utc>,
}

fn free_promo_ends_at(
    offer: &epic::schema::Offer,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<chrono::DateTime<chrono::Utc>> {
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

            let (Some(start), Some(end)) = (
                time::parse_utc(&promo.start_date),
                time::parse_utc(&promo.end_date),
            ) else {
                continue;
            };

            if start <= now && now < end {
                return Some(end);
            }
        }
    }

    None
}

pub fn get_free_offers(
    ts: &impl time::TimeSource,
    ec: &impl epic::http::HttpClient,
    existing_offer_ids: std::collections::HashMap<String, i64>,
) -> Result<Vec<Offer>, Box<dyn std::error::Error>> {
    let now = ts.now();

    let body = ec.fetch_offers().map_err(|e| {
        tracing::error!("Failed to fetch offers from Epic Games Store: {e}");
        e
    })?;
    let root = serde_json::from_str::<epic::schema::Response>(&body).map_err(|e| {
        tracing::error!("Failed to parse Epic Games Store response: {e}");
        e
    })?;

    let offers: Vec<Offer> = root
        .data
        .catalog
        .search_store
        .elements
        .into_iter()
        .filter_map(|offer| {
            let ends_at = free_promo_ends_at(&offer, now)?;

            if existing_offer_ids.contains_key(&offer.id) {
                tracing::info!(title = %offer.title, "Offer already posted, skipping");
                return None;
            }

            let is_bundle = offer
                .offer_type
                .is_some_and(|ot| ot == epic::schema::OfferType::Bundle)
                || offer.categories.is_some_and(|cats| {
                    cats.iter()
                        .any(|ct| ct.path == "bundles" || ct.path == "bundles/games")
                });

            let store_link = match offer.product_slug {
                Some(slug) if is_bundle => {
                    format!("https://store.epicgames.com/en-US/bundles/{slug}")
                }
                Some(slug) => format!("https://www.epicgames.com/store/en-US/p/{slug}"),
                None => return None,
            };

            Some(Offer {
                id: offer.id,
                source: epic::SOURCE.to_string(),
                title: offer.title,
                link: store_link,
                ends_at,
            })
        })
        .collect();

    Ok(offers)
}

pub fn handle(
    ts: &impl time::TimeSource,
    ec: &impl epic::http::HttpClient,
    store: &impl offer_store::OfferStore,
    n: &dyn notifier::Notifier,
) -> Result<(), Box<dyn std::error::Error>> {
    let existing_offers = store.get_existing_offers()?;

    let epic_free_offers = get_free_offers(
        ts,
        ec,
        existing_offers
            .into_iter()
            .filter(|o| o.source == epic::SOURCE)
            .map(|o| (o.id, o.ends_at))
            .collect(),
    )?;

    for offer in epic_free_offers {
        let ends_unix = offer.ends_at.timestamp();
        let ends_rel = format!("<t:{ends_unix}:R>");

        let message = format!(
            "**{}** is now free on {}! Ends {}\n{}",
            offer.title, offer.source, ends_rel, offer.link,
        );

        n.notify(&message)?;
        store.insert_offer(&offer.id, &offer.source, ends_unix)?;
    }

    Ok(())
}
