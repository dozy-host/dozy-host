use bollard::Docker;

pub struct Container {
    pub id: str
}

impl Container {
    pub async fn start(&self, docker: &Docker) -> anyhow::Result<()> {
        docker.start_container::<String>(&self.id, None).await?;
        Ok(())
    }

    pub async fn stop(&self, docker: &Docker) -> anyhow::Result<()> {
        docker.stop_container(&self.id, None).await?;
        Ok(())
    }

    pub async fn remove(&self, docker: &Docker) -> anyhow::Result<()> {
        docker.remove_container(&self.id, None).await?;
        Ok(())
    }

    pub async fn logs(&self, docker: &Docker) -> anyhow::Result<String> {
        

        let logs = docker.logs::<String>(&self.id, None).await?;
        Ok(logs)
    }
}