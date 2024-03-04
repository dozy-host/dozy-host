mod discord;
mod docker;
mod credentials;

pub use discord::login;
use log::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    info!("Hello, world!");

    let docker = docker::connect_to_docker()?;

    if let Err(e) = login(discord::Data {
        docker,
    }).await {
        panic!("Failed to login: {}", e);
    }

    info!("Successfully logged in!");

    Ok(())
}