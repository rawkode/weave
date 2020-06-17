use std::{collections::HashSet, path::PathBuf};

pub fn detect_build_roots(changed_dirs: HashSet<PathBuf>) -> bool {
    for directory in changed_dirs {
        log::info!("Got a changed dir {} ", directory.to_str().unwrap())
    }

    return false;
}
