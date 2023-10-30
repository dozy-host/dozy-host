use std::path::Path;

use bollard::{Docker, API_DEFAULT_VERSION};

use crate::configuration::docker::DockerConfig;

pub fn connect(config: DockerConfig) -> Result<Docker, bollard::errors::Error> {
    Docker::connect_with_ssl("cog.dragon-bortle.ts.net", Path::new(""), Path::new(""), Path::new(""), 120, API_DEFAULT_VERSION)
}