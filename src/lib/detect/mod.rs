use std::{collections::HashSet, path::PathBuf};

/// This function will walk up the directory tree, to the path
/// the binary was run from; looking for any kind of build file.
/// Maybe this should walk until it finds a Git root, but I am
/// keen to avoid tying this to Git; even though it's the first,
/// and potentially only, integration.
pub fn detect_build_roots(changed_dirs: HashSet<PathBuf>) -> bool {
    for directory in changed_dirs {
        log::info!("Got a changed dir {} ", directory.to_str().unwrap())
    }

    return false;
}
