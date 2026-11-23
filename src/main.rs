use cordex::prelude::*;
use tracing_subscriber::EnvFilter;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let token = std::env::var("DISCORD_TOKEN")
        .expect("set DISCORD_TOKEN in your environment (see .env.example)");

    Bot::new(token)
        .command(
            Command::slash("ping", "Replies with pong"),
            |ctx: CommandContext| async move { ctx.reply("Pong!").await },
        )
        .on_ready(|ctx: ReadyContext| async move {
            tracing::info!("connected as {}", ctx.user().username);
            Ok(())
        })
        .run()
        .await?;

    Ok(())
}
