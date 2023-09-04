use super::client::{Context, Error};

type CommandResult = Result<(), Error>;

#[poise::command(slash_command)]
pub async fn start(
    ctx: Context<'_>,
) -> CommandResult {
    

    Ok(())
}