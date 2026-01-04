use free_games_notifier::offer_store::OfferStore;
use free_games_notifier::{app, epic, notifier, offer_store, time};

#[cfg(test)]
mod fixture_tests {
    use super::*;

    fn setup(
        s: &str,
        resp: &str,
    ) -> (
        time::MockTimeSource,
        epic::http::MockClient,
        offer_store::SqliteOfferStore,
        notifier::CaptureNotifier,
    ) {
        let ts = time::MockTimeSource::new(time::parse_utc(s).unwrap());

        let ec = epic::http::MockClient::new(resp.to_string());

        let db = rusqlite::Connection::open_in_memory().unwrap();
        let offer_store = offer_store::SqliteOfferStore::new(db);
        offer_store.ensure_schema().unwrap();

        let n = notifier::CaptureNotifier::new();

        (ts, ec, offer_store, n)
    }

    #[test]
    fn test_epic_handle_single_promo() {
        let (ts, ec, offer_store, n) = setup(
            "2025-12-20T16:15:00.000Z",
            include_str!("./fixtures/epic_single_promo.json"),
        );

        app::epic::handle(&ts, &ec, &offer_store, &n).unwrap();

        let msgs: std::collections::HashSet<String> = n.get_messages().into_iter().collect();
        let expected: std::collections::HashSet<String> = [
            "**Blood West** is now free on EGS! Ends <t:1766332800:R>\nhttps://www.epicgames.com/store/en-US/p/blood-west-8f6ffd"
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(msgs, expected);
    }

    #[test]
    fn test_epic_handle_bundle_promo() {
        let (ts, ec, offer_store, n) = setup(
            "2025-12-30T16:00:00.000Z",
            include_str!("./fixtures/epic_bundle_promo.json"),
        );

        app::epic::handle(&ts, &ec, &offer_store, &n).unwrap();

        let msgs: std::collections::HashSet<String> = n.get_messages().into_iter().collect();
        let expected: std::collections::HashSet<String> = [
            "**Trine Classic Collection** is now free on EGS! Ends <t:1767196800:R>\nhttps://store.epicgames.com/en-US/bundles/trine-classic-collection"
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(msgs, expected);
    }

    #[test]
    fn test_epic_handle_multiple_promo() {
        let (ts, ec, offer_store, n) = setup(
            "2026-01-01T16:15:00.000Z",
            include_str!("./fixtures/epic_multiple_promo.json"),
        );

        app::epic::handle(&ts, &ec, &offer_store, &n).unwrap();

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

    #[test]
    fn test_epic_handle_multiple_promo_same_multi_run() {
        let (ts, ec, offer_store, n) = setup(
            "2026-01-01T16:15:00.000Z",
            include_str!("./fixtures/epic_multiple_promo.json"),
        );

        app::epic::handle(&ts, &ec, &offer_store, &n).unwrap();

        // First run should emit both messages, refer to `test_epic_handle_multiple_promo` for details.
        assert_eq!(
            n.get_messages().len(),
            2,
            "sanity check: first run should emit"
        );

        let n2 = notifier::CaptureNotifier::new();
        // Second run against SAME offer_store/db -> should emit nothing.
        app::epic::handle(&ts, &ec, &offer_store, &n2).unwrap();

        assert!(
            n2.get_messages().is_empty(),
            "expected no notifications on second run; got {:?}",
            n2.get_messages()
        );
    }
}
