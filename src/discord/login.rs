use super::{commands, Data};
use anyhow::Error;
use poise::serenity_prelude::{self as serenity};

pub async fn login(data: Data) -> Result<(), Error> {
    let token = crate::credentials::discord_token();

    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands(),
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .run()
        .await?;
    Ok(())
}
