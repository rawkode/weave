use std::{collections::HashSet, path::PathBuf};

use crate::lib::build_tools::docker::default as DockerDefault;
use crate::lib::build_tools::gitlab::default as GitLabDefault;
use crate::lib::build_tools::{BuildConfig, BuildTool, BuildTools};

/// This function will walk up the directory tree, to the path
/// the binary was run from; looking for any kind of build file.
/// Maybe this should walk until it finds a Git root, but I am
/// keen to avoid tying this to Git; even though it's the first,
/// and potentially only, integration.
pub fn detect_build_roots(root: &PathBuf, changed_dirs: &HashSet<PathBuf>) -> HashSet<BuildTools> {
    let mut build_roots: HashSet<BuildTools> = HashSet::new();

    for dir in changed_dirs {
        log::info!(
            "Walking from {} to {}, looking for a build root ...",
            dir.to_str().unwrap(),
            root.to_str().unwrap(),
        );
        match walk_to_build_root(root, &dir) {
            Some(build_tool) => {
                build_roots.insert(build_tool);
            }
            None => {}
        }
    }

    return build_roots;
}

///
/// This function will recurse over the directories between
/// the changed_dir and the root directory, looking for
/// build files
///
fn walk_to_build_root(root: &PathBuf, changed_dir: &PathBuf) -> Option<BuildTools> {
    log::info!(
        "Checking {}/{} for a build file",
        root.to_str().unwrap(),
        changed_dir.to_str().unwrap()
    );

    let build_config = BuildConfig {
        directory: root.join(changed_dir.to_path_buf()),
        dependencies: Vec::new(),
    };

    // Check if it's a Docker root
    if DockerDefault(&build_config).detect() {
        log::info!(
            "Found a Dockerfile inside directory {}",
            build_config.directory.to_str().unwrap()
        );
    }

    // Check if it's a GitLab root
    if GitLabDefault(&build_config).detect() {
        log::info!(
            "Found a GitLab CI YAML inside directory {}",
            build_config.directory.to_str().unwrap()
        );
    }

    if root.eq(changed_dir) {
        return None;
    }

    let mut dir = changed_dir.to_owned();

    return match dir.pop() {
        true => walk_to_build_root(root, &dir),
        false => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_can_find_all_build_roots() {
        let mut changed_dirs: HashSet<PathBuf> = HashSet::new();
        changed_dirs.insert(PathBuf::from("./examples/dockerfile"));
        changed_dirs.insert(PathBuf::from("./examples/gitlab-ci"));
        changed_dirs.insert(PathBuf::from("./examples/makefile"));

        let actual = detect_build_roots(&PathBuf::from("./examples"), &changed_dirs);

        let expected: HashSet<BuildTools> = HashSet::new();

        assert_eq!(actual, expected);
    }
}
