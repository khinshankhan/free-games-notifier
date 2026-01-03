use offer_store::OfferStore;
use time::TimeSource;

mod app;
mod discord;
mod epic;
mod notifier;
mod offer_store;
mod time;

fn get_notifier() -> Box<dyn notifier::Notifier> {
    let webhook_url = match std::env::var("DISCORD_WEBHOOK_URL") {
        Ok(url) => url,
        Err(_) => {
            tracing::warn!("DISCORD_WEBHOOK_URL not set, falling back to logging notifier.");
            return Box::new(notifier::LoggingNotifier);
        }
    };

    return Box::new(discord::DiscordNotifier::new(webhook_url));
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let ts = time::SystemTimeSource;

    let db = rusqlite::Connection::open("offers.db")?;
    let offer_store = offer_store::SqliteOfferStore::new(db);
    offer_store.ensure_schema()?;
    offer_store.prune_expired_offers(ts.now().timestamp())?;

    let ec = epic::RealClient;
    let n = get_notifier();

    app::handle_epic(&ts, &ec, &offer_store, &*n)?;

    Ok(())
}

fn main()  {
    tracing_subscriber::fmt::init();

    match run() {
        Ok(()) => println!("Successfully fetched and displayed relevant offers."),
        Err(e) => {
            tracing::error!("Application error: {}", e);
            std::process::exit(1);
        },
    }
}
