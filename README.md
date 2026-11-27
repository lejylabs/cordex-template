# cordex-template

Minimal `cargo-generate` template for a Discord bot built with [cordex](https://github.com/lejylabs/cordex).

## Usage

```bash
cargo install cargo-generate
cargo generate lejylabs/cordex-template
```

Then configure your bot token and run:

```bash
cp .env.example .env
# edit .env and export DISCORD_TOKEN
cargo run
```

## Included

- `cordex` 0.2.3 with `Bot::from_env()`, `sync_dev_guild_from_env()`, `/ping` & `/greet` slash commands, `MessageBuilder`, `on_error`, and graceful `Ctrl+C` shutdown
- Offline unit tests with `cordex-test` (`TestBot`)
- `tracing` + `tracing-subscriber` (`RUST_LOG=info`)
- `mimalloc` global allocator
- Optimized `[profile.release]` (`lto = "fat"`, `codegen-units = 1`, `panic = "unwind"`, `cordex-model` size-optimized)
- GitHub Actions CI (`fmt`, `clippy`, `test`)
