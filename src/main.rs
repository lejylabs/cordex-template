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
        .slash("ping", "Replies with pong", |ctx| async move {
            ctx.reply("Pong!").await
        })
        .command(
            Command::slash("greet", "Greets a user").required_user("user", "Who to greet"),
            |ctx| async move {
                let user = ctx.user_option("user")?;
                let text = MessageBuilder::new()
                    .push("Hello ")
                    .mention(&user.id)
                    .push("!")
                    .build();
                ctx.reply(text).await
            },
        )
        .on_ready(|ctx| async move {
            tracing::info!("connected as {}", ctx.user().username);
            Ok(())
        })
        .on_error(|_ctx, err| async move {
            tracing::error!(%err, "handler error");
        })
        .run_until_ctrl_c()
        .await?;

    Ok(())
}
