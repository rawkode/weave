use super::{BuildConfig, BuildTool};

pub struct DockerBuild {
    config: BuildConfig,
    dockerfile: String,
}

pub fn default(config: &BuildConfig) -> DockerBuild {
    return DockerBuild {
        config: config.clone(),
        dockerfile: String::from("Dockerfile"),
    };
}

impl BuildTool for DockerBuild {
    fn id(&self) -> String {
        return format!(
            "{}/{}",
            self.config.directory.to_str().unwrap(),
            self.dockerfile
        );
    }

    fn detect(&self) -> bool {
        // Check if PathBuf has a file called Dockerfile
        if self
            .config
            .directory
            .join(self.dockerfile.as_str())
            .exists()
        {
            return true;
        }

        return false;
    }

    fn build(&self) -> bool {
        return true;
    }
}
