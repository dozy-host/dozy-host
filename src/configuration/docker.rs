use std::path::PathBuf;

use bollard::{Docker, API_DEFAULT_VERSION};
use serde::*;

pub trait DockerConnectionConfig {
    fn connect(&self) -> Result<Docker, bollard::errors::Error>;
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DockerConfig {
    socket: String,
}

impl Default for DockerConfig {
    fn default() -> Self {
        Self {
            socket: "unix:///var/run/docker.sock".to_string(),
        }
    }
}

impl DockerConnectionConfig for DockerConfig {
    fn connect(&self) -> Result<Docker, bollard::errors::Error> {
        Docker::connect_with_socket(&self.socket, 120, API_DEFAULT_VERSION)
    }
}

pub struct HttpsDockerConfig {
    pub host: String,
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
    pub ca_path: PathBuf,
}

impl DockerConnectionConfig for HttpsDockerConfig {
    fn connect(&self) -> Result<Docker, bollard::errors::Error> {
        Docker::connect_with_ssl(&self.host, &self.cert_path, &self.key_path, &self.ca_path, 120, API_DEFAULT_VERSION)
    }
}

pub struct HttpDockerConfig {
    pub host: String,
}

impl DockerConnectionConfig for HttpDockerConfig {
    fn connect(&self) -> Result<Docker, bollard::errors::Error> {
        Docker::connect_with_http(&self.host, 120, API_DEFAULT_VERSION)
    }
}