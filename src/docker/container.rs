use std::rc::Rc;

use bollard::Docker;

struct Container {
    pub name: String,
    pub docker: Rc<Docker>,
}

impl Container {
    pub fn new(name: String, docker: Rc<Docker>) -> Self {
        Self {
            name,
            docker,
        }
    }

    pub async fn start(&self) -> Result<(), bollard::errors::Error> {
        self.docker.start_container::<String>(&self.name, None).await
    }

    pub async fn stop(&self) -> Result<(), bollard::errors::Error> {
        self.docker.stop_container(&self.name, None).await
    }
}