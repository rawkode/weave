use std::cmp::Eq;
use std::hash::Hash;
use std::{fmt::Debug, path::PathBuf};

use docker::DockerBuild;
use gitlab::GitLabBuild;

pub mod docker;
pub mod gitlab;

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
    fn detect(&self) -> bool;
    fn build(&self) -> bool;
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum BuildTools {
    Docker(DockerBuild),
    GitLab(GitLabBuild),
}
