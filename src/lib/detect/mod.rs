use std::{collections::HashSet, path::PathBuf};

use crate::lib::build_tools::{BuildConfig, BuildTools, DEFAULT_BUILD_TOOLS};

/// This function will walk up the directory tree, to the path
/// the binary was run from; looking for any kind of build file.
/// Maybe this should walk until it finds a Git root, but I am
/// keen to avoid tying this to Git; even though it's the first,
/// and potentially only, integration.
pub fn detect_build_roots(root: &PathBuf, changed_dirs: &HashSet<PathBuf>) -> HashSet<BuildTools> {
    let mut build_roots: HashSet<BuildTools> = HashSet::new();
    for dir in changed_dirs {
        log::debug!(
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
    log::debug!(
        "Checking {}/{} for a build file",
        root.to_str().unwrap(),
        changed_dir.to_str().unwrap()
    );

    let build_config = BuildConfig {
        directory: root.join(changed_dir.to_path_buf()),
        dependencies: Vec::new(),
    };

    for detect in DEFAULT_BUILD_TOOLS.iter() {
        match detect(&build_config) {
            Some(build_tool) => return Some(build_tool),
            None => continue,
        }
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
    use crate::lib::build_tools::docker::DockerBuild;
    use crate::lib::build_tools::gitlab::GitLabBuild;

    #[test]
    fn test_is_can_find_all_build_roots() {
        let root_dir = PathBuf::from("examples");

        let mut changed_dirs: HashSet<PathBuf> = HashSet::new();
        changed_dirs.insert(PathBuf::from("dockerfile"));
        changed_dirs.insert(PathBuf::from("gitlab-ci"));
        changed_dirs.insert(PathBuf::from("makefile"));

        let actual = detect_build_roots(&root_dir, &changed_dirs);

        let mut expected: HashSet<BuildTools> = HashSet::new();

        expected.insert(BuildTools::from(GitLabBuild {
            config: BuildConfig {
                directory: PathBuf::from("examples/gitlab-ci"),
                dependencies: Vec::new(),
            },
        }));

        expected.insert(BuildTools::from(DockerBuild {
            config: BuildConfig {
                directory: PathBuf::from("examples/dockerfile"),
                dependencies: Vec::new(),
            },
            dockerfile: String::from("Dockerfile"),
        }));

        assert_eq!(actual, expected);
    }
}
