use git2::{Error, Repository};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

///
/// Verifies if the directory is a valid Git repository by
/// attempting to open it. Will return an Error for bare
/// and empty repositories.
///
pub fn verify(directory: &PathBuf) -> Result<Repository, Error> {
    let repository = match Repository::open(directory) {
        Err(e) => return Err(e),
        Ok(r) => r,
    };

    // Bare repositories aren't handled because we can't build if
    // there's no code on the disk.
    match repository.is_bare() {
        true => return Err(Error::from_str("Weave cannot build a bare repository")),
        false => (),
    }

    // Empty repositories aren't handled because we can't build nothing.
    match repository.is_empty().unwrap() {
        true => return Err(Error::from_str("Weave cannot build an empty repository")),
        false => (),
    }

    return Ok(repository);
}

///
/// Detects changes in the Git repository by walking back
/// to the previous commit.
///
pub fn detect(directory: &PathBuf) -> Result<HashSet<PathBuf>, Error> {
    let repo = match verify(directory) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let rev = revwalk.find(|walk| {
        let rev = match walk {
            &Ok(r) => r,
            &Err(_) => return true,
        };
        match repo.find_commit(rev) {
            Ok(_commit) => return true,
            Err(_) => return false,
        };
    });

    let commit = match repo.find_commit(rev.unwrap()?) {
        Ok(commit) => commit,
        Err(e) => return Err(e),
    };

    let parent_commit = commit.parent(0)?;

    let diff = repo
        .diff_tree_to_tree(
            Some(&commit.tree().unwrap()),
            Some(&parent_commit.tree().unwrap()),
            None,
        )
        .unwrap();
    let ds = diff.deltas();

    let mut dirs = HashSet::new();

    for d in ds {
        let dir = d.new_file().path().unwrap().parent().unwrap().to_path_buf();
        // dirs.insert(directory.join(dir));
        dirs.insert(dir);
    }

    return Ok(dirs);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir, File};
    use std::path::Path;
    use tempfile::TempDir;

    macro_rules! t {
        ($e:expr) => {
            match $e {
                Ok(e) => e,
                Err(e) => panic!("{} failed with {}", stringify!($e), e),
            }
        };
    }

    fn repo_init() -> (TempDir, Repository) {
        let td = TempDir::new().unwrap();
        let repo = Repository::init(td.path()).unwrap();

        let mut index = t!(repo.index());
        let sig = t!(repo.signature());

        let mut config = t!(repo.config());
        t!(config.set_str("user.name", "Daniel Jackson"));
        t!(config.set_str("user.email", "daniel.jackson@sg1"));

        // This is our initial commit. It addds 1 file within 1 directory
        // This directory shouldn't show up during change analysis of
        // subsequent test commits
        {
            t!(create_dir(&td.path().join("first")));

            t!(File::create(&td.path().join("first/file")));
            t!(index.add_path(Path::new("first/file")));

            let id = t!(index.write_tree());
            let tree = t!(repo.find_tree(id));

            t!(repo.commit(Some("HEAD"), &sig, &sig, "commit", &tree, &[]));
        }

        (td, repo)
    }

    #[test]
    fn test_directory_is_bare() {
        let td = TempDir::new().unwrap();
        let path = td.path();
        Repository::init_bare(td.path()).unwrap();

        assert_eq!(verify(&path.to_path_buf()).is_err(), true)
    }

    #[test]
    fn test_directory_is_empty() {
        let td = TempDir::new().unwrap();
        let path = td.path();
        Repository::init(td.path()).unwrap();

        assert_eq!(verify(&path.to_path_buf()).is_err(), true)
    }

    #[test]
    fn test_is_can_verify_git_repository() {
        let (td, _repo) = repo_init();

        assert_eq!(verify(&td.path().to_path_buf()).is_err(), false)
    }

    #[test]
    fn test_can_discover_modified_directories() {
        let (td, repo) = repo_init();

        let mut index = t!(repo.index());
        let sig = t!(repo.signature());

        let mut modpaths: HashSet<PathBuf> = HashSet::new();
        modpaths.insert(PathBuf::from("second"));
        modpaths.insert(PathBuf::from("third"));

        // Add 2 directories, each with a file
        {
            for path in modpaths.iter() {
                t!(create_dir(&td.path().join(path)));
                t!(File::create(&td.path().join(path).join("file")));
                t!(index.add_path(&path.join("file")));
            }

            let id = t!(index.write_tree());
            let tree = t!(repo.find_tree(id));

            let head_id = t!(repo.refname_to_id("HEAD"));
            let parent = t!(repo.find_commit(head_id));

            t!(repo.commit(Some("HEAD"), &sig, &sig, "commit", &tree, &[&parent]));
        }

        let changedirs = t!(detect(&td.path().to_path_buf()));

        assert_eq!(changedirs, modpaths);
    }
}
