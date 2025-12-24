mod epic;

use epic::EpicResponse;

fn get_epic_data() -> Result<String, reqwest::Error> {
    let epic_url =
        "https://store-site-backend-static.ak.epicgames.com/freeGamesPromotions?locale=en-US";

    Ok(reqwest::blocking::get(epic_url)?.text()?)
}

fn handle_epic() -> Result<(), Box<dyn std::error::Error>> {
    let body = get_epic_data()?;
    let root = serde_json::from_str::<EpicResponse>(&body)?;

    let offers = root.data.catalog.search_store.elements;
    for offer in offers.iter() {
        println!(
            "{} ({}:{}) discountPrice={}",
            offer.title,
            offer.namespace,
            offer.id,
            offer.price.total_price.discount_price
        );
    }

    Ok(())
}

fn main() {
    match handle_epic() {
        Ok(()) => println!("Successfully fetched and displayed Epic Games offers."),
        Err(e) => eprintln!("HTTP error: {e}"),
    }
}
