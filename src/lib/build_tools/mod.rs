use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

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

impl Hash for Box<dyn BuildTool> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id().hash(state)
    }
}

impl PartialEq for Box<dyn BuildTool> {
    fn eq(&self, other: &Box<dyn BuildTool>) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Box<dyn BuildTool> {}
