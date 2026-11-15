# free-games-notifier

Notifies about currently free games to claim.

> [!WARNING]
> This was made mostly for fun and as an exercise to play with Rust.

## Usage

Create a `settings.toml` next to where you run the binary:

```toml
[[discord.targets]]
id = "friends"
webhook_url = "https://discord.com/api/webhooks/first-id/first-token"

[[discord.targets]]
id = "work"
webhook_url = "https://discord.com/api/webhooks/second-id/second-token"
```

Then hook it up to a cronjob. It should send a message with the game name and a link to redeem it.

Config loading is runtime-based:

- `./free-games-notifier --config /path/to/settings.toml` uses the exact file you pass in.
- Without `--config`, the app searches for the nearest `settings.toml` starting from the current working directory and walking upward.
- If both `/srv/settings.toml` and `/srv/prod/free-games/settings.toml` exist, running from `/srv/prod/free-games` uses the more specific local file.
- If no config file is found, startup fails because at least one `discord.targets` entry is required.
- The SQLite database path remains fixed at `offers.db` for now.
- Each `discord.targets` entry needs a unique `id`. That `id` is what the app stores in SQLite to decide whether a given server has already been notified about an offer.
- The app assumes the target-aware SQLite schema. Older `offers.db` files from before `target_id` support are not migrated automatically.

An example file lives at [`settings.toml.example`](./settings.toml.example).

> [!TIP]
> It's likely better to run the app with logs/ tracing enabled (`RUST_LOG=debug`) since there are a few useful logs like
> skipping if the offer was already posted (helpful to know why nothing was printed).

### Docker

The image runs the notifier on a schedule with [supercronic](https://github.com/aptible/supercronic). Nothing
deployment-specific is baked in, so mount your own files:

- `/etc/free-games-notifier/settings.toml` -- your config (see `settings.toml.example`).
- `/etc/free-games-notifier/crontab` -- your schedule (see `crontab.example`).
- `/data` -- a persistent volume; `offers.db` lives here.

Set `TZ` (eg `TZ=Europe/Berlin`) to write the schedule in a timezone other than UTC.

```sh
docker build -t free-games-notifier .
docker run -d \
  -v free-games-notifier-data:/data \
  -v "$PWD/settings.toml:/etc/free-games-notifier/settings.toml:ro" \
  -v "$PWD/crontab:/etc/free-games-notifier/crontab:ro" \
  free-games-notifier
```

## Contributing

Bug reports and PRs are welcome. Please open an issue first for discussion.

Feel free to open an issue if you spot something iffy or have a hot tip :shrug:

## License

Apache-2.0. See [LICENSE](./LICENSE). If applicable, see [NOTICE](./NOTICE).
