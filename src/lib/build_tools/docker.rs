use super::{BuildConfig, BuildTool, BuildTools};
use std::process::Command;
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    io,
    io::Write,
    path::PathBuf,
};

#[derive(Eq)]
pub struct DockerBuild {
    pub config: BuildConfig,
    pub dockerfile: String,
}

///
/// Detect if the directory has a Dockerfile
///
pub fn detect(config: &BuildConfig) -> Option<BuildTools> {
    log::debug!(
        "Checking for '{}'",
        config.directory.join("Dockerfile").to_str().unwrap()
    );

    if config.directory.join("Dockerfile").exists() {
        return Some(BuildTools::from(DockerBuild {
            config: config.clone(),
            dockerfile: String::from("Dockerfile"),
        }));
    }

    return None;
}

impl BuildTool for DockerBuild {
    fn id(&self) -> String {
        return format!(
            "{}/{}",
            self.config.directory.to_str().unwrap(),
            self.dockerfile
        );
    }

    fn build(&self) -> bool {
        // This won't work on Windows
        // TODO: Allow `docker` to be buildah, etc.
        let output = Command::new("docker")
            .arg("image")
            .arg("build")
            .arg("-f")
            .arg(PathBuf::from(&self.config.directory).join(&self.dockerfile))
            .arg("-t")
            .arg("weave_build")
            .arg(self.config.directory.to_str().unwrap())
            .output()
            .expect("failed to execute process");

        if !output.status.success() {
            log::error!("Failed to execute Docker build\n\n\n");
            io::stderr().write_all(&output.stderr).unwrap();

            return false;
        }

        println!("Build Complete, created image weave_build");

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
