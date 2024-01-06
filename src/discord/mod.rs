mod commands;
mod login;

use anyhow::Error;
pub use commands::commands;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub use login::login;