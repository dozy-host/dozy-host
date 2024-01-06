use std::rc::Rc;

use bollard::{Docker, container::LogsOptions};
use futures::{Stream, StreamExt};

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

    pub async fn create(&self) -> Result<(), bollard::errors::Error> {
        let config = bollard::container::Config {
            image: Some(self.name.clone()),

            ..Default::default()
        };


        self.docker.create_container::<String, String>(
            None,
            bollard::container::Config {
                image: Some(self.name.clone()),
                ..Default::default()
            }
        ).await.map(|_| ())
    }

    pub async fn test(&self) -> Result<(), bollard::errors::Error> {
        let mut stream = self.docker.events::<String>(None);
        let result = stream.next().await.unwrap().unwrap();

        Ok(())
    }
}