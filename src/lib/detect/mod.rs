use std::{collections::HashSet, path::PathBuf};

/// This function will walk up the directory tree, to the path
/// the binary was run from; looking for any kind of build file.
/// Maybe this should walk until it finds a Git root, but I am
/// keen to avoid tying this to Git; even though it's the first,
/// and potentially only, integration.
pub fn detect_build_roots(changed_dirs: &HashSet<PathBuf>) -> HashSet<PathBuf> {
    let mut build_roots: HashSet<PathBuf> = HashSet::new();

    for dir in changed_dirs {
        log::info!("Walking from {} to {}, looking for a build root ...", dir.to_str().unwrap(), PathBuf::from(".").to_str().unwrap());
        build_roots.insert(walk_to_build_root(dir));
    }

    return build_roots;
}

///
/// This function will recurse over the directories between
/// the changed_dir and the root directory, looking for
/// build files
///
/// If it cannot find any, it will return the root directory
///
fn walk_to_build_root(changed_dir: &PathBuf) -> PathBuf {
    log::info!("Checking ./{} for a build file", changed_dir.to_str().unwrap());

    let mut dir = changed_dir.to_owned();

    return match dir.pop() {
        true => walk_to_build_root(&dir),
        false => dir,
    };
}

