use docker::{detect as detect_docker, DockerBuild};
use gitlab::{detect as detect_gitlab, GitLabBuild};
use std::cmp::Eq;
use std::hash::Hash;
use std::{fmt::Debug, path::PathBuf};

pub mod docker;
pub mod gitlab;

// This order is important, this is the order they'll stop matching
pub const DEFAULT_BUILD_TOOLS: [fn(&BuildConfig) -> Option<BuildTools>; 2] =
    [detect_gitlab, detect_docker];

#[derive(Clone, Eq, Hash)]
pub struct BuildConfig {
    pub directory: PathBuf,
    pub dependencies: Vec<BuildConfig>,
}

impl PartialEq for BuildConfig {
    fn eq(&self, other: &Self) -> bool {
        return self.directory == other.directory && self.dependencies == other.dependencies;
    }
}

pub trait BuildTool {
    fn id(&self) -> String;
    fn build(&self) -> bool;
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum BuildTools {
    Docker(DockerBuild),
    GitLab(GitLabBuild),
}
