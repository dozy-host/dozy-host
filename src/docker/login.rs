use anyhow::Error;
use bollard::Docker;

pub fn connect_to_docker() -> anyhow::Result<Docker> {
    let docker = Docker::connect_with_socket_defaults()?;
    Ok(docker)
}