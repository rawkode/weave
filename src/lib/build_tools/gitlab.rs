use super::{BuildConfig, BuildTool};

pub struct GitLabBuild {
    config: BuildConfig,
}

pub fn default(config: &BuildConfig) -> GitLabBuild {
    return GitLabBuild {
        config: config.clone(),
    };
}

impl BuildTool for GitLabBuild {
    fn id(&self) -> String {
        return String::from(self.config.directory.to_str().unwrap());
    }

    fn detect(&self) -> bool {
        // Check if PathBuf has a file called .gitlab-ci.yml
        log::info!(
            "Checking for gitlab-ci.yml inside {}",
            self.config.directory.to_str().unwrap()
        );
        if self.config.directory.join(".gitlab-ci.yml").exists() {
            return true;
        }

        return false;
    }

    fn build(&self) -> bool {
        return true;
    }
}
