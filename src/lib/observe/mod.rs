use std::{collections::HashSet, path::PathBuf};

pub mod all;
pub mod git;

pub trait Observer {
    fn observe(&self) -> HashSet<PathBuf>;
}
