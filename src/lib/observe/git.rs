use git2::{Error, Repository};
use std::{collections::HashSet, path::PathBuf};

///
/// Verifies if the directory is a valid Git repository by
/// attempting to open it. Will return an Error for bare
/// and empty repositories.
///
pub fn verify(directory: &str) -> Result<Repository, Error> {
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
pub fn detect(directory: &str) -> Result<HashSet<PathBuf>, Error> {
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
        dirs.insert(dir);
    }

    return Ok(dirs);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_directory_isnt_repository() {
        assert_eq!(verify("/tmp").is_err(), true)
    }

    #[test]
    fn test_directory_is_bare() {
        let td = TempDir::new().unwrap();
        let path = td.path();
        Repository::init_bare(td.path()).unwrap();

        assert_eq!(verify(path.to_str().unwrap()).is_err(), true)
    }

    #[test]
    fn test_directory_is_empty() {
        let td = TempDir::new().unwrap();
        let path = td.path();
        Repository::init(td.path()).unwrap();

        assert_eq!(verify(path.to_str().unwrap()).is_err(), true)
    }

    // #[test]
    // fn test_is_can_verify_git_repository() {
    //     let td = TempDir::new().unwrap();
    //     let path = td.path();
    //     let repo = Repository::init(td.path()).unwrap();

    //     let mut tmpfile: File = tempfile::tempfile().unwrap();
    //     write!(tmpfile, "Initial File!").unwrap();

    //     let mut index = repo.index().unwrap();
    //     let tree_id = {
    //         index.add_path(tmpfile);
    //         index.write_tree().unwrap();
    //     };

    //     let tree = repo.find_tree(tree_id)?;
    //     repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

    //     assert_eq!(verify(".").is_ok(), true)
    // }
}
