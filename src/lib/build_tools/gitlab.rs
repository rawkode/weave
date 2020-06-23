use super::{BuildConfig, BuildTool, BuildTools};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(Eq)]
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

impl From<GitLabBuild> for BuildTools {
    fn from(v: GitLabBuild) -> Self {
        BuildTools::GitLab(v)
    }
}

impl Hash for GitLabBuild {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id().hash(state)
    }
}

impl PartialEq for GitLabBuild {
    fn eq(&self, other: &GitLabBuild) -> bool {
        return self.id() == other.id();
    }
}

impl Debug for GitLabBuild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitLabBuild")
            .field("config.directory", &self.config.directory)
            .finish()
    }
}
