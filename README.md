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

- `cordex` 0.1.1 with `Bot` and a `/ping` slash command
- `tracing` + `tracing-subscriber` (`RUST_LOG=info`)
- `mimalloc` global allocator
- Optimized `[profile.release]` (`lto = "fat"`, `codegen-units = 1`, `panic = "unwind"`)
- GitHub Actions CI (`fmt`, `clippy`, `test`)
