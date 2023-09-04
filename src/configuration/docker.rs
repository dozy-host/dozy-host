use serde::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct DockerConfig {
    socket: String,
}