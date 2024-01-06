#![deny(clippy::use_self)]

use crate::configuration::docker::{HttpDockerConfig, DockerConnectionConfig, DockerConfig};

mod configuration;
mod docker;
mod discord;
mod host;
mod event;
mod mediator;
mod communication;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    println!("Token: {}", token);

    // Tls not working because of a "Bad Certificate" error

    // let docker_config = HttpsDockerConfig {
    //     host: "tcp://cog.dragon-bortle.ts.net:2376".to_string(),
    //     cert_path: PathBuf::from("/home/crystal/.docker/cog-cert.pem"),
    //     key_path: PathBuf::from("/home/crystal/.docker/cog-key.pem"),
    //     ca_path: PathBuf::from("/home/crystal/.docker/cog-ca.pem"),
    // };

    // let docker = docker_config.connect_with_ssl().unwrap();

    // let docker_config = HttpDockerConfig {
    //     host: "tcp://cog.dragon-bortle.ts.net:2375".to_string(),
    // };

    let discord_client = discord::client::create_client(token).await;

    let docker_config = DockerConfig::default();

    let docker = docker_config.connect().unwrap();

    println!("Docker connection successful, API version {:?}", docker.version().await.unwrap().api_version);
}
