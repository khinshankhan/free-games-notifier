use free_games_notifier::offer_store::OfferStore;
use free_games_notifier::{app, epic, notifier, offer_store, time};

#[cfg(test)]
mod fixture_tests {
    use super::*;
    #[test]
    fn test_handle_epic() {
        let ts = time::MockTimeSource::new(time::parse_utc("2026-01-02T16:00:00.000Z").unwrap());

        let resp = include_str!("./fixtures/epic_multiple_promo.json");
        let ec = epic::http::MockClient::new(resp.to_string());

        let db = rusqlite::Connection::open_in_memory().unwrap();
        let offer_store = offer_store::SqliteOfferStore::new(db);
        offer_store.ensure_schema().unwrap();

        let n = notifier::CaptureNotifier::new();

        app::handle_epic(&ts, &ec, &offer_store, &n).unwrap();

        let msgs: std::collections::HashSet<String> = n.get_messages().into_iter().collect();
        let expected: std::collections::HashSet<String> = [
            "**Wildgate** is now free on EGS! Ends <t:1767888000:R>\n\
https://www.epicgames.com/store/en-US/p/wildgate-standard-edition-b886b5",
            "**Total War: THREE KINGDOMS** is now free on EGS! Ends <t:1767888000:R>\n\
https://www.epicgames.com/store/en-US/p/total-war-three-kingdoms-d3bb7a",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(msgs, expected);
    }
}
