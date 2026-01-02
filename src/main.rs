use offer_store::OfferStore;
use time::TimeSource;

mod discord;
mod epic;
mod epic_client;
mod epic_logic;
mod notifier;
mod offer_store;
mod time;

fn get_notifier() -> Box<dyn notifier::Notifier> {
    let allow_post_flag = std::env::var("ALLOW_POST_FLAG")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase();

    if allow_post_flag != "true" {
        return Box::new(notifier::LoggingNotifier);
    }

    let webhook_url = match std::env::var("DISCORD_WEBHOOK_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("DISCORD_WEBHOOK_URL not set, falling back to logging notifier.");
            return Box::new(notifier::LoggingNotifier);
        }
    };

    return Box::new(discord::DiscordNotifier::new(webhook_url));
}

fn main() {
    dotenvy::dotenv().ok();

    let ts = time::SystemTimeSource;

    let db = rusqlite::Connection::open("offers.db").expect("Failed to open database");
    let offer_store = offer_store::SqliteOfferStore::new(db);
    offer_store
        .ensure_schema()
        .expect("Failed to ensure database schema");
    offer_store
        .prune_expired_offers(ts.now().timestamp())
        .expect("Failed to prune expired offers");

    let ec = epic_client::RealClient;
    let n = get_notifier();

    match epic_logic::handle_epic(&ts, &ec, &offer_store, &n) {
        Ok(()) => println!("Successfully fetched and displayed Epic Games offers."),
        Err(e) => eprintln!("HTTP error: {e}"),
    }
}
