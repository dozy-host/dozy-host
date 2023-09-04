use super::commands::*;
use std::sync::Arc;

use log::warn;
use poise::serenity_prelude::{self as serenity, GatewayIntents};
use tokio::sync::RwLock;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    host_adapters: RwLock<Vec<Arc<dyn DiscordAdapter>>>,
}

pub enum Interaction {}

trait DiscordAdapter: Send + Sync {
    fn handle_interaction(&self, interaction: Interaction) -> Result<(), ()>;
}

async fn create_client() {
    let options = poise::FrameworkOptions {
        commands: vec![start()],

        ..Default::default()
    };

    poise::Framework::builder()
        .token("N/A")
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                for guild in &ready.guilds {
                    match poise::builtins::register_in_guild(ctx, &framework.options().commands, guild.id).await {
                        Err(err) => {
                            warn!("Unable")
                        },
                        _ => {}
                    }
                }
                Ok(Data {
                    host_adapters: RwLock::new(vec![]),
                })
            })
        })
        .options(options)
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .run()
        .await
        .unwrap();
}
