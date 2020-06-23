use super::Observer;
use std::{collections::HashSet, path::PathBuf};
use walkdir::WalkDir;

pub struct AllConfig {
    pub directory: PathBuf,
}

impl Observer for AllConfig {
    fn observe(&self) -> HashSet<PathBuf> {
        let mut obs: HashSet<PathBuf> = HashSet::new();
        log::info!(
            "Searching for all inside {}",
            self.directory.to_str().unwrap()
        );

        for entry in WalkDir::new(&self.directory) {
            match entry {
                Ok(e) => match e.path().is_dir() {
                    true => {
                        obs.insert(e.path().to_path_buf());
                    }
                    false => (),
                },
                Err(e) => {
                    log::error!("Failed to scan directory {}", e);
                    continue;
                }
            };
        }

        return obs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_can_find_all_directories() {
        let all_config: AllConfig = AllConfig {
            directory: PathBuf::from("./examples"),
        };

        let actual = all_config.observe();

        let mut expected: HashSet<PathBuf> = HashSet::new();

        expected.insert(PathBuf::from("./examples"));
        expected.insert(PathBuf::from("./examples/dockerfile"));
        expected.insert(PathBuf::from("./examples/gitlab-ci"));
        expected.insert(PathBuf::from("./examples/makefile"));

        assert_eq!(expected.eq(&actual), true);
    }
}
