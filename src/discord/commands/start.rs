use super::Context;
use anyhow::Error;

/// Start the container with the provided short name
#[poise::command(slash_command)]
pub async fn start(
    ctx: Context<'_>,
    #[description = "Short name of the server to start"] short_name: String,
) -> Result<(), Error> {
    ctx.data().docker.start_container::<String>(&short_name, None).await?;
    let response = format!("Starting {}...", short_name);
    ctx.say(response).await?;
    Ok(())
}
