pub fn discord_token() -> String {
    std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN")
}