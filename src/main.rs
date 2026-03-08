use free_games_notifier::offer_store::OfferStore;
use free_games_notifier::time::TimeSource;
use free_games_notifier::{app, config, epic, notifier, offer_store, time};

fn get_notifier(config: &config::Config) -> Box<dyn notifier::Notifier> {
    let webhook_url = match config.discord.webhook_url.clone() {
        Some(url) => url,
        None => {
            tracing::warn!(
                "discord.webhook_url not configured, falling back to logging notifier."
            );
            return Box::new(notifier::LoggingNotifier);
        }
    };

    return Box::new(notifier::DiscordNotifier::new(webhook_url));
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config_load = config::Config::from_cli_args(std::env::args_os())?;
    let config = config_load.config;

    match config_load.path {
        Some(path) => tracing::info!("Using config file at {}", path.display()),
        None => tracing::info!(
            "No {} found in the current directory tree. Using built-in defaults.",
            config::DEFAULT_CONFIG_FILE_NAME
        ),
    }

    let ts = time::SystemTimeSource;

    let db = rusqlite::Connection::open("offers.db")?;
    let offer_store = offer_store::SqliteOfferStore::new(db);
    offer_store.ensure_schema()?;
    offer_store.prune_expired_offers(ts.now().timestamp())?;

    let ec = epic::http::Client;
    let n = get_notifier(&config);

    app::epic::handle(&ts, &ec, &offer_store, &*n)?;

    Ok(())
}

fn main() {
    tracing_subscriber::fmt::init();

    match run() {
        Ok(()) => {
            tracing::info!("Application finished successfully.");
        }
        Err(e) => {
            tracing::error!("Application error: {}", e);
            std::process::exit(1);
        }
    }
}
