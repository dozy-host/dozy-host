use bollard::Docker;

use crate::configuration::docker::DockerConfig;

pub fn connect(config: DockerConfig) -> Result<Docker, bollard::errors::Error> {
    Docker::connect_with_socket_defaults()
}