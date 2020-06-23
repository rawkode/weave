use super::{BuildConfig, BuildTool, BuildTools};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(Eq)]
pub struct GitLabBuild {
    pub config: BuildConfig,
}

///
/// Detect if the directory has a .gitlab-ci.yml
///
pub fn detect(config: &BuildConfig) -> Option<BuildTools> {
    log::debug!(
        "Checking for '{}'",
        config.directory.join(".gitlab-ci.yml").to_str().unwrap()
    );

    if config.directory.join(".gitlab-ci.yml").exists() {
        return Some(BuildTools::from(GitLabBuild {
            config: config.clone(),
        }));
    }

    return None;
}

impl BuildTool for GitLabBuild {
    fn id(&self) -> String {
        return String::from(self.config.directory.to_str().unwrap());
    }

    fn build(&self) -> bool {
        println!(
            "This can only be built using GitLab CI, and currently I can't output it's config\n\n"
        );
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
