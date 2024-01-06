pub mod discord;

pub use discord::login;
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Hello, world!");

    if let Err(e) = login(discord::Data {}).await {
        panic!("Failed to login: {}", e);
    }

    info!("Successfully logged in!")
}