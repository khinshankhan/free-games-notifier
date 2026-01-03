# free-games-notifier

Notifies about currently free games to claim.

> [!WARNING]
> This was made mostly for fun and as an exercise to play with Rust.

## Usage

Throw in your discord webhook url into an `.env` likeso:

```bash
DISCORD_WEBHOOK_URL="<your actual url>"
```

and hook it up to a cronjob. It should send a message with the game name and a link to redeem it.

> [!TIP]
> It's likely better to run the app with logs/ tracing enabled (`RUST_LOG=debug`) since there are a few useful logs like
> skipping if the offer was already posted (helpful to know why nothing was printed).

## Contributing

Bug reports and PRs are welcome. Please open an issue first for discussion.

Feel free to open an issue if you spot something iffy or have a hot tip :shrug:

## License

Apache-2.0. See [LICENSE](./LICENSE). If applicable, see [NOTICE](./NOTICE).
