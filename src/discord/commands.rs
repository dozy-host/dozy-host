use super::client::{Context, Error};

type CommandResult = Result<(), Error>;

/// Start an existing container
#[poise::command(slash_command)]
pub async fn start(
    ctx: Context<'_>,
    #[description = "The name of the container to start"] container_name: String,
) -> CommandResult {
    let response = format!("Starting container {}", container_name);
    ctx.say(response).await?;
    Ok(())
}

/// Stop an existing container
#[poise::command(slash_command)]
pub async fn stop(
    ctx: Context<'_>,
    #[description = "The name of the container to stop"] container_name: String,
) -> CommandResult {
    let response = format!("Stopping container {}", container_name);
    ctx.say(response).await?;
    Ok(())
}
