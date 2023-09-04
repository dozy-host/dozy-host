#![deny(clippy::use_self)]

mod configuration;
mod docker;
mod discord;
mod host;
mod event;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    
}
