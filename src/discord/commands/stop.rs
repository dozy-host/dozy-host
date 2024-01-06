use super::{Context, Data};
use anyhow::Error;

/// Stop the container with the provided short name
#[poise::command(slash_command)]
pub async fn stop(
    ctx: Context<'_>,
    #[description = "Short name of the server to stop"] short_name: String,
) -> Result<(), Error> {
    let response = format!("Stopping {}...", short_name);
    ctx.say(response).await?;
    Ok(())
}
