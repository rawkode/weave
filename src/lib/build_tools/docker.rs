use super::{BuildConfig, BuildTool, BuildTools};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(Eq)]
pub struct DockerBuild {
    pub config: BuildConfig,
    pub dockerfile: String,
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

impl From<DockerBuild> for BuildTools {
    fn from(v: DockerBuild) -> Self {
        BuildTools::Docker(v)
    }
}

impl Hash for DockerBuild {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id().hash(state)
    }
}

impl PartialEq for DockerBuild {
    fn eq(&self, other: &DockerBuild) -> bool {
        self.id() == other.id()
    }
}

impl Debug for DockerBuild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DockerBuild")
            .field("config.directory", &self.config.directory)
            .field("dockerfile", &self.dockerfile)
            .finish()
    }
}
