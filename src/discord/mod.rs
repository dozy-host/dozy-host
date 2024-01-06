use crate::communication::Origin;

pub mod client;
pub mod commands;

const DISCORD_ORIGIN_STR: &str = "discord";
pub static DISCORD_ORIGIN: Origin = Origin::register(DISCORD_ORIGIN_STR);