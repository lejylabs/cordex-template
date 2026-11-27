use cordex::prelude::*;
use tracing_subscriber::EnvFilter;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn configure(bot: Bot) -> Bot {
    bot.slash("ping", "Replies with pong", |ctx| async move {
        ctx.reply("Pong!").await
    })
    .command(
        Command::slash("greet", "Greets a user")
            .required_user("user", "Who to greet")
            .string("message", "Optional custom greeting"),
        |ctx| async move {
            let user = ctx.user_option("user")?;
            let greeting = ctx.string_option_opt("message")?.unwrap_or("Hello");
            let text = MessageBuilder::new()
                .push(greeting)
                .push(" ")
                .mention(&user.id)
                .push("!")
                .build();
            ctx.reply(text).await
        },
    )
    .on_ready(|ctx| async move {
        tracing::info!("connected as {}", ctx.bot_user().username);
        Ok(())
    })
    .on_error(|_ctx, err| async move {
        tracing::error!(%err, "handler error");
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    configure(Bot::from_env()?)
        .sync_dev_guild_from_env()
        .run_until_ctrl_c()
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use cordex_test::TestBot;

    use super::*;

    #[tokio::test]
    async fn ping_replies_with_pong() -> Result<(), Box<dyn std::error::Error>> {
        let harness = TestBot::new(configure(Bot::new("test"))).await?;
        harness.slash("ping").send().await.assert_replied("Pong!");
        Ok(())
    }

    #[tokio::test]
    async fn greet_mentions_target_user() -> Result<(), Box<dyn std::error::Error>> {
        let harness = TestBot::new(configure(Bot::new("test"))).await?;
        let user_id = UserId::new(400_000_000_000_000_042).expect("non-zero snowflake");

        harness
            .slash("greet")
            .user_opt("user", user_id)
            .string_opt("message", "Welcome")
            .send()
            .await
            .assert_replied("Welcome <@400000000000000042>!");
        Ok(())
    }
}
