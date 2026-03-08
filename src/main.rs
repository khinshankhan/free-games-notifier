use free_games_notifier::offer_store::OfferStore;
use free_games_notifier::time::TimeSource;
use free_games_notifier::{app, config, epic, notifier, offer_store, time};

fn get_notifiers(
    config: &config::Config,
) -> Result<Vec<(String, Box<dyn notifier::Notifier>)>, Box<dyn std::error::Error>> {
    let targets = config.discord.targets();

    if targets.is_empty() {
        return Err("no discord.targets configured".into());
    }

    let mut seen_target_ids = std::collections::HashSet::new();

    targets
        .into_iter()
        .map(|target| {
            if !seen_target_ids.insert(target.id.clone()) {
                return Err(
                    format!("duplicate discord target id configured: {}", target.id).into(),
                );
            }

            Ok((
                target.id,
                Box::new(notifier::DiscordNotifier::new(target.webhook_url))
                    as Box<dyn notifier::Notifier>,
            ))
        })
        .collect()
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
    let notifiers = get_notifiers(&config)?;
    let configured_target_ids = notifiers
        .iter()
        .map(|(id, _)| id.as_str())
        .collect::<Vec<_>>();
    tracing::info!(targets = ?configured_target_ids, "Configured Discord targets");
    let notify_targets = notifiers
        .iter()
        .map(|(id, notifier)| app::epic::NotifyTarget {
            id: id.as_str(),
            notifier: notifier.as_ref(),
        })
        .collect::<Vec<_>>();

    app::epic::handle(&ts, &ec, &offer_store, &notify_targets)?;

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
